package kz.legis.entropy.service;

import co.elastic.clients.elasticsearch.ElasticsearchClient;
import co.elastic.clients.elasticsearch._types.FieldValue;
import co.elastic.clients.elasticsearch._types.aggregations.Aggregate;
import co.elastic.clients.elasticsearch._types.aggregations.StringTermsBucket;
import co.elastic.clients.elasticsearch.core.BulkRequest;
import co.elastic.clients.elasticsearch.core.SearchRequest;
import co.elastic.clients.elasticsearch.core.search.Hit;
import co.elastic.clients.json.JsonData;
import jakarta.annotation.PostConstruct;
import kz.legis.entropy.domain.CorpusAggregations;
import kz.legis.entropy.domain.FacetedSearchResult;
import kz.legis.entropy.domain.GraphData;
import kz.legis.entropy.domain.LegalDocumentDoc;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Mono;
import reactor.core.scheduler.Schedulers;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Collectors;

@Service
public class ElasticsearchService {

    private static final Logger log = LoggerFactory.getLogger(ElasticsearchService.class);
    private static final String INDEX = "legal_documents";

    private final ElasticsearchClient client;

    public ElasticsearchService(ElasticsearchClient client) {
        this.client = client;
    }

    @PostConstruct
    public void ensureIndex() {
        Mono.fromCallable(this::createIndexIfAbsent)
                .subscribeOn(Schedulers.boundedElastic())
                .subscribe(
                        created -> { if (created) log.info("ES index '{}' created", INDEX); },
                        e -> log.warn("ES not reachable on startup — will retry on first use: {}", e.getMessage())
                );
    }

    private boolean createIndexIfAbsent() throws Exception {
        boolean exists = client.indices().exists(r -> r.index(INDEX)).value();
        if (!exists) {
            client.indices().create(r -> r
                    .index(INDEX)
                    .mappings(m -> m
                            .properties("title",       p -> p.text(t -> t))
                            .properties("url",         p -> p.keyword(k -> k))
                            .properties("status",      p -> p.keyword(k -> k))
                            .properties("issueKinds",  p -> p.keyword(k -> k))
                            .properties("isAmendment", p -> p.boolean_(b -> b))
                            .properties("refCount",    p -> p.integer(i -> i))
                            .properties("issueCount",  p -> p.integer(i -> i))
                            .properties("articleCount",p -> p.integer(i -> i))
                    )
            );
            return true;
        }
        return false;
    }

    /**
     * Bulk-index all graph nodes into Elasticsearch.
     * Called explicitly via POST /api/es/reindex.
     */
    public Mono<Long> indexDocuments(GraphData graph) {
        Map<String, List<String>> docIssueKinds = new HashMap<>();
        for (var issue : graph.issues()) {
            for (var docId : issue.documentIds()) {
                docIssueKinds.computeIfAbsent(docId, k -> new ArrayList<>()).add(issue.kind());
            }
        }

        List<LegalDocumentDoc> docs = graph.nodes().stream()
                .map(node -> new LegalDocumentDoc(
                        node.id(), node.title(), node.url(), node.status(),
                        node.refCount(), node.issueCount(), node.articleCount(),
                        node.isAmendment(),
                        List.copyOf(docIssueKinds.getOrDefault(node.id(), List.of()))
                ))
                .toList();

        return Mono.fromCallable(() -> {
            if (docs.isEmpty()) return 0L;

            var builder = new BulkRequest.Builder();
            for (var doc : docs) {
                final var d = doc;
                builder.operations(op -> op.index(idx -> idx
                        .index(INDEX)
                        .id(d.id())
                        .document(d)
                ));
            }

            var response = client.bulk(builder.build());
            long indexed = response.items().stream()
                    .filter(item -> item.error() == null)
                    .count();
            long failed = docs.size() - indexed;
            if (failed > 0) log.warn("ES bulk: {} docs failed to index", failed);
            log.info("ES reindex complete: {} docs indexed", indexed);
            return indexed;
        })
        .subscribeOn(Schedulers.boundedElastic())
        .onErrorResume(e -> {
            log.error("ES indexing error: {}", e.getMessage());
            return Mono.error(new RuntimeException("Elasticsearch unavailable: " + e.getMessage(), e));
        });
    }

    /**
     * Faceted search: BM25 on title, optional filters on status / hasIssues / isAmendment.
     * Aggregation buckets (status, issueKind) are returned alongside hits.
     */
    public Mono<FacetedSearchResult> facetedSearch(
            String query, String status, Boolean hasIssues, Boolean isAmendment, int page, int size) {
        return Mono.fromCallable(() -> {
            var request = SearchRequest.of(s -> s
                    .index(INDEX)
                    .query(q -> q.bool(b -> {
                        if (query != null && !query.isBlank()) {
                            b.must(m -> m.match(match -> match.field("title").query(query)));
                        } else {
                            b.must(m -> m.matchAll(ma -> ma));
                        }
                        if (status != null && !status.isBlank()) {
                            b.filter(f -> f.term(t -> t.field("status").value(FieldValue.of(status))));
                        }
                        if (Boolean.TRUE.equals(hasIssues)) {
                            b.filter(f -> f.range(r -> r
                                    .field("issueCount").gte(JsonData.of(1))));
                        }
                        if (isAmendment != null) {
                            final boolean val = isAmendment;
                            b.filter(f -> f.term(t -> t.field("isAmendment").value(FieldValue.of(val))));
                        }
                        return b;
                    }))
                    .aggregations("by_status",     a -> a.terms(t -> t.field("status").size(20)))
                    .aggregations("by_issue_kind", a -> a.terms(t -> t.field("issueKinds").size(20)))
                    .from(page * size)
                    .size(size)
            );

            var response = client.search(request, LegalDocumentDoc.class);

            List<LegalDocumentDoc> hits = response.hits().hits().stream()
                    .map(Hit::source)
                    .filter(Objects::nonNull)
                    .toList();
            long total = response.hits().total() != null ? response.hits().total().value() : 0L;

            Map<String, Long> statusBuckets    = extractStringTermBuckets(response.aggregations(), "by_status");
            Map<String, Long> issueKindBuckets = extractStringTermBuckets(response.aggregations(), "by_issue_kind");

            return new FacetedSearchResult(hits, total, statusBuckets, issueKindBuckets);
        })
        .subscribeOn(Schedulers.boundedElastic());
    }

    /**
     * Corpus aggregations: totals, breakdown by status and issue kind,
     * count of docs with issues, count of amendments.
     */
    public Mono<CorpusAggregations> getCorpusAggregations() {
        return Mono.fromCallable(() -> {
            var request = SearchRequest.of(s -> s
                    .index(INDEX)
                    .size(0)
                    .aggregations("by_status",     a -> a.terms(t -> t.field("status").size(20)))
                    .aggregations("by_issue_kind", a -> a.terms(t -> t.field("issueKinds").size(20)))
                    .aggregations("with_issues",   a -> a.filter(f -> f.range(r -> r
                            .field("issueCount").gte(JsonData.of(1)))))
                    .aggregations("amendments",    a -> a.filter(f -> f.term(t -> t
                            .field("isAmendment").value(FieldValue.of(true)))))
            );

            var response = client.search(request, LegalDocumentDoc.class);
            long total = response.hits().total() != null ? response.hits().total().value() : 0L;

            Map<String, Long> byStatus    = extractStringTermBuckets(response.aggregations(), "by_status");
            Map<String, Long> byIssueKind = extractStringTermBuckets(response.aggregations(), "by_issue_kind");
            long withIssues = extractFilterDocCount(response.aggregations(), "with_issues");
            long amendments = extractFilterDocCount(response.aggregations(), "amendments");

            return new CorpusAggregations(total, byStatus, byIssueKind, withIssues, amendments);
        })
        .subscribeOn(Schedulers.boundedElastic());
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    private Map<String, Long> extractStringTermBuckets(Map<String, Aggregate> aggs, String name) {
        if (aggs == null || !aggs.containsKey(name)) return Map.of();
        Aggregate agg = aggs.get(name);
        if (!agg.isSterms()) return Map.of();
        return agg.sterms().buckets().array().stream()
                .collect(Collectors.toMap(
                        b -> b.key().stringValue(),
                        StringTermsBucket::docCount
                ));
    }

    private long extractFilterDocCount(Map<String, Aggregate> aggs, String name) {
        if (aggs == null || !aggs.containsKey(name)) return 0L;
        Aggregate agg = aggs.get(name);
        if (!agg.isFilter()) return 0L;
        return agg.filter().docCount();
    }
}
