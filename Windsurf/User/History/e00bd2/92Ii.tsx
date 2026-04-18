import { useState } from 'react';
import type { CorpusStats, GraphNode, Issue } from '../types';

interface SidebarProps {
  node: GraphNode | null;
  issues: Issue[];
  stats: CorpusStats | null;
  onClose: () => void;
  onCompareSelect: (node: GraphNode) => void;
  compareNode: GraphNode | null;
}

const STATUS_LABEL: Record<string, string> = {
  active: 'Действующий',
  outdated: 'Утратил силу',
  unknown: 'Неизвестен',
};

const STATUS_DOT: Record<string, string> = {
  active: 'bg-gray-800',
  outdated: 'bg-gray-600',
  unknown: 'bg-gray-400',
};

const STATUS_STYLE: Record<string, string> = {
  active: 'text-gray-800 bg-gray-100 border-gray-300',
  outdated: 'text-gray-700 bg-gray-100 border-gray-300',
  unknown: 'text-gray-500 bg-gray-100 border-gray-200',
};

const ISSUE_KIND_LABEL: Record<string, string> = {
  duplication: 'Дублирование',
  contradiction: 'Противоречие',
  outdated_reference: 'Устаревшая ссылка',
  circular_reference: 'Циклические ссылки',
  amendment: 'Акт внесения изменений',
};

const ISSUE_KIND_ICON: Record<string, string> = {
  duplication: '⊙',
  contradiction: '⚡',
  outdated_reference: '🔗',
  circular_reference: '↺',
  amendment: '✎',
};

const SEVERITY_STYLE: Record<string, string> = {
  high: 'text-gray-800 bg-gray-100 border-gray-300',
  medium: 'text-gray-700 bg-gray-100 border-gray-300',
  low: 'text-gray-600 bg-gray-100 border-gray-200',
};

const SEVERITY_LABEL: Record<string, string> = {
  high: 'Высокий',
  medium: 'Средний',
  low: 'Низкий',
};


const EXPLANATION_LIMIT = 160;

function IssueCard({ issue }: { issue: Issue; index: number }) {
  const [expanded, setExpanded] = useState(false);
  const long = issue.explanation.length > EXPLANATION_LIMIT;
  const text = !expanded && long
    ? issue.explanation.slice(0, EXPLANATION_LIMIT) + '…'
    : issue.explanation;

  return (
    <div className={`rounded-xl border p-5 space-y-3 ${
      issue.kind === 'amendment' ? 'border-gray-300 bg-gray-100' : 'border-gray-200 bg-gray-50'
    }`}>
      <div className="flex items-start justify-between gap-3">
        <div className="flex items-center gap-3 min-w-0 flex-1">
          <span className="text-base leading-none shrink-0 mt-0.5">{ISSUE_KIND_ICON[issue.kind] ?? '⚠'}</span>
          <div className="min-w-0 flex-1">
            <span className="text-[11px] font-semibold text-gray-800 block leading-tight">
              {ISSUE_KIND_LABEL[issue.kind] ?? issue.kind}
            </span>
          </div>
        </div>
        <span className={`text-[9px] font-semibold px-2.5 py-1 rounded-full border shrink-0 ${SEVERITY_STYLE[issue.severity]}`}>
          {SEVERITY_LABEL[issue.severity]}
        </span>
      </div>
      <div className="pl-9">
        <p className="text-[11px] text-gray-500 leading-relaxed">{text}</p>
        {long && (
          <button
            onClick={() => setExpanded(e => !e)}
            className="text-[9px] font-medium text-gray-400 hover:text-gray-600 transition-colors mt-2"
          >
            {expanded ? '↑ Свернуть' : '↓ Подробнее'}
          </button>
        )}
        <div className="text-[9px] text-gray-400 mt-2">
          Затронуто документов: {issue.document_ids.length}
        </div>
      </div>
    </div>
  );
}

export function Sidebar({ node, issues, stats, onClose, onCompareSelect, compareNode }: SidebarProps) {
  const nodeIssues = node
    ? issues.filter(i => i.document_ids.includes(node.id))
    : [];

  return (
    <div className="absolute top-0 right-0 h-full w-80 flex flex-col z-10 pointer-events-none">
    <div className="flex flex-col h-full bg-white border-l border-gray-200 pointer-events-auto">

      {/* Header */}
      <div className="px-8 py-6 border-b border-gray-100 flex items-center justify-between shrink-0">
        <div className="flex items-center gap-2.5">
          <div className="w-1 h-4 rounded-full bg-gray-800" />
          <span className="text-xs font-bold text-gray-900 tracking-tight">Инспектор</span>
        </div>
        {stats && (
          <span className="text-[10px] text-gray-400 tabular-nums">
            {stats.total_documents} НПА
          </span>
        )}
      </div>

      <div className="flex-1 overflow-y-auto min-h-0">
        {node ? (
          <div className="p-8 space-y-6">
            {/* Title + close */}
            <div className="flex items-start justify-between gap-2">
              <h2 className="text-sm font-medium text-gray-900 leading-snug">{node.title}</h2>
              <button
                onClick={onClose}
                className="shrink-0 w-5 h-5 flex items-center justify-center rounded-md text-gray-400 hover:text-gray-700 hover:bg-gray-100 transition-colors"
              >
                <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                  <path d="M1 1l8 8M9 1L1 9" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
                </svg>
              </button>
            </div>

            {/* Badges row */}
            <div className="flex items-center gap-1.5 flex-wrap">
              <span className={`inline-flex items-center gap-1.5 px-4 py-1 rounded-full text-[10px] font-medium border ${STATUS_STYLE[node.status]}`}>
                <span className={`w-1.5 h-1.5 rounded-full ${STATUS_DOT[node.status]}`} />
                {STATUS_LABEL[node.status]}
              </span>
              {node.is_amendment && (
                <span className="inline-flex items-center gap-1 px-4 py-1 rounded-full text-[10px] font-medium text-gray-700 bg-gray-100 border border-gray-300">
                  ✎ Акт изменений
                </span>
              )}
              {node.issue_count > 0 && (
                <span className="inline-flex items-center gap-1 px-4 py-1 rounded-full text-[10px] font-medium text-gray-700 bg-gray-100 border border-gray-300">
                  ⚠ {node.issue_count} {node.issue_count === 1 ? 'проблема' : 'проблем'}
                </span>
              )}
            </div>

            {/* Metadata table */}
            <div className="rounded-lg border border-gray-100 divide-y divide-gray-100 bg-gray-50/50">
              <div className="flex items-center justify-between px-6 py-4">
                <span className="text-[10px] text-gray-400">ID документа</span>
                <code className="text-[10px] text-gray-600 font-mono bg-gray-100 px-3 py-1 rounded">{node.id}</code>
              </div>
              <div className="flex items-center justify-between px-6 py-4">
                <span className="text-[10px] text-gray-400">Ссылки на НПА</span>
                <span className="text-[10px] text-gray-700 font-medium">{node.ref_count} <span className="font-normal text-gray-400">документов</span></span>
              </div>
              {node.article_count > 0 && (
                <div className="flex items-center justify-between px-6 py-4">
                  <span className="text-[10px] text-gray-400">Ключевые нормы</span>
                  <span className="text-[10px] text-gray-700 font-medium">{node.article_count} <span className="font-normal text-gray-400">статей</span></span>
                </div>
              )}
              <div className="flex items-center justify-between px-6 py-4">
                <span className="text-[10px] text-gray-400">Тип документа</span>
                <span className="text-[10px] text-gray-600">
                  {node.is_amendment ? 'Акт внесения изменений' : node.article_count > 10 ? 'Кодекс / закон' : 'НПА'}
                </span>
              </div>
            </div>

            {/* Issues */}
            {nodeIssues.length > 0 && (
              <div className="space-y-3">
                <p className="text-[10px] font-bold text-gray-400 uppercase tracking-widest pl-2 border-l-2 border-gray-200">
                  Почему помечен ({nodeIssues.length})
                </p>
                {nodeIssues.map((issue, i) => (
                  <IssueCard key={i} issue={issue} index={i} />
                ))}
              </div>
            )}

            {/* Actions */}
            <div className="flex gap-2 pt-2">
              <a
                href={node.url}
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 flex items-center justify-center gap-1.5 text-[11px] font-medium py-4 px-6 rounded-lg bg-gray-900 hover:bg-gray-700 text-white transition-colors"
              >
                Открыть
                <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                  <path d="M2 8L8 2M8 2H4M8 2v4" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" strokeLinejoin="round"/>
                </svg>
              </a>
              <button
                onClick={() => onCompareSelect(node)}
                className={`flex items-center gap-1.5 text-[11px] font-medium py-4 px-6 rounded-lg border transition-all ${
                  compareNode?.id === node.id
                    ? 'bg-gray-900 border-gray-900 text-white'
                    : 'border-gray-200 text-gray-600 hover:border-gray-300 hover:text-gray-900'
                }`}
              >
                {compareNode?.id === node.id ? (
                  <>
                    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                      <path d="M1.5 5l2.5 2.5 4.5-5" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" strokeLinejoin="round"/>
                    </svg>
                    Выбран
                  </>
                ) : 'Сравнить'}
              </button>
            </div>
          </div>
        ) : (
          /* Issues overview */
          <div className="p-8 space-y-6">
            <div>
              <p className="text-xs font-medium text-gray-500 mb-0.5">Выберите узел на графе</p>
              <p className="text-[10px] text-gray-400">Ctrl+клик — сравнить два документа</p>
            </div>

            {issues.length > 0 ? (
              <div className="space-y-3">
                <p className="text-[10px] font-bold text-gray-400 uppercase tracking-widest pl-2 border-l-2 border-gray-200">
                  Обнаруженные проблемы ({issues.length})
                </p>
                {issues.map((issue, i) => (
                  <IssueCard key={i} issue={issue} index={i} />
                ))}
              </div>
            ) : (
              <div className="rounded-lg border border-emerald-100 bg-gray-100 p-5">
                <p className="text-[11px] font-medium text-gray-700">Проблем не обнаружено</p>
                <p className="text-[10px] text-gray-600 mt-1">Корпус документов согласован</p>
              </div>
            )}
          </div>
        )}

        {/* Corpus stats */}
        {stats && (
          <div className="px-8 pb-8 space-y-4 border-t border-gray-100 pt-8">
            <p className="text-[10px] font-bold text-gray-400 uppercase tracking-widest pl-2 border-l-2 border-gray-200">Состояние корпуса</p>

            <div className="grid grid-cols-2 gap-3">
              {[
                { label: 'Действующих', value: stats.active_count, colorClass: 'text-gray-800', accentColor: '#000000' },
                { label: 'Утративших', value: stats.outdated_count, colorClass: 'text-gray-600', accentColor: '#666666' },
                { label: 'С проблемами', value: stats.with_issues, colorClass: 'text-gray-700', accentColor: '#333333' },
                { label: 'Ср. ссылок', value: stats.avg_ref_count.toFixed(1), colorClass: 'text-gray-700', accentColor: '#999999' },
              ].map(({ label, value, colorClass, accentColor }) => (
                <div key={label} className="rounded-lg border border-gray-100 bg-gray-50/50 p-4 overflow-hidden relative">
                  <div className="absolute top-0 left-0 right-0 h-0.5" style={{ background: accentColor }} />
                  <div className={`text-lg font-bold font-mono leading-none ${colorClass}`}>{value}</div>
                  <div className="text-[10px] text-gray-400 mt-1">{label}</div>
                </div>
              ))}
            </div>

            {stats.most_problematic.length > 0 && (
              <div className="space-y-1">
                <p className="text-[10px] text-gray-400">Наиболее проблемные</p>
                {stats.most_problematic.slice(0, 3).map(n => (
                  <div key={n.id} className="flex items-center justify-between gap-2 py-3 border-b border-gray-100 last:border-0">
                    <span className="text-[10px] text-gray-600 truncate flex-1">{n.title}</span>
                    <span className="text-[10px] text-gray-600 font-semibold font-mono shrink-0">{n.issue_count}</span>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

    </div>
    </div>
  );
}
