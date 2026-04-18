import { useState, useRef, useEffect } from 'react';
import { Graph } from './components/Graph';
import { Sidebar } from './components/Sidebar';
import { SearchBar } from './components/SearchBar';
import { AnalysisProgress } from './components/AnalysisProgress';
import { ComparePanel } from './components/ComparePanel';
import { Logo } from './components/Logo';
import { useGraphData } from './hooks/useGraphData';
import { useAnalysisStream } from './hooks/useAnalysisStream';
import type { GraphNode } from './types';

const LEGEND_ITEMS = [
  { color: '#000000', label: 'Действующий' },
  { color: '#666666', label: 'Утратил силу' },
  { color: '#999999', label: 'Неизвестен' },
  { color: '#333333', label: 'Акт изменений' },
];

export default function App() {
  const { data, loading, error, refetch } = useGraphData();
  const { state: stream, start: startStream, stop: stopStream } = useAnalysisStream();

  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
  const [compareNode, setCompareNode] = useState<GraphNode | null>(null);
  const [searchHits, setSearchHits] = useState<string[]>([]);
  const [corpusReview, setCorpusReview] = useState<{ review: string; llm_ready: boolean } | null>(null);
  const [corpusReviewLoading, setCorpusReviewLoading] = useState(false);

  const fetchCorpusReview = async () => {
    setCorpusReviewLoading(true);
    try {
      const res = await fetch('/api/corpus-review');
      if (res.ok) setCorpusReview(await res.json());
    } finally {
      setCorpusReviewLoading(false);
    }
  };

  // Track Ctrl/Meta key globally — canvas events sometimes lose ctrlKey
  const ctrlHeld = useRef(false);
  useEffect(() => {
    const down = (e: KeyboardEvent) => { if (e.key === 'Control' || e.key === 'Meta') ctrlHeld.current = true; };
    const up = (e: KeyboardEvent) => { if (e.key === 'Control' || e.key === 'Meta') ctrlHeld.current = false; };
    window.addEventListener('keydown', down);
    window.addEventListener('keyup', up);
    return () => { window.removeEventListener('keydown', down); window.removeEventListener('keyup', up); };
  }, []);

  const activeData = stream.graph ?? data;
  const activeStats = stream.stats ?? null;

  const handleNodeClick = (node: GraphNode, event?: MouseEvent) => {
    const isCtrl = event?.ctrlKey || event?.metaKey || ctrlHeld.current;
    if (isCtrl) {
      setCompareNode(prev => prev?.id === node.id ? null : node);
      return;
    }
    setSelectedNode(prev => prev?.id === node.id ? null : node);
  };

  const handleCompareSelect = (node: GraphNode) => {
    setCompareNode(prev => prev?.id === node.id ? null : node);
  };

  if (error && !activeData) {
    return (
      <div className="h-full flex items-center justify-center bg-white">
        <div className="text-center space-y-4 p-16 rounded-2xl border border-gray-200 shadow-sm">
          <div className="w-10 h-10 rounded-full bg-red-50 border border-red-100 flex items-center justify-center mx-auto">
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none">
              <path d="M9 6v3m0 3h.01M17 9A8 8 0 111 9a8 8 0 0116 0z" stroke="#ef4444" strokeWidth="1.5" strokeLinecap="round"/>
            </svg>
          </div>
          <div>
            <p className="text-sm font-medium text-gray-900">Не удалось загрузить граф</p>
            <p className="text-xs text-gray-400 mt-1">{error}</p>
          </div>
          <button
            onClick={refetch}
            className="px-8 py-3 text-xs font-medium bg-gray-900 hover:bg-gray-700 text-white rounded-xl transition-colors"
          >
            Повторить
          </button>
        </div>
      </div>
    );
  }

  const showCompare = !!(compareNode && selectedNode && compareNode.id !== selectedNode.id);

  return (
    <div className="relative h-full w-full bg-gradient-to-br from-slate-50 via-white to-slate-100">

      {/* Loading overlay */}
      {loading && !activeData && (
        <div className="absolute inset-0 flex items-center justify-center z-20 bg-white/80 backdrop-blur-md">
          <div className="text-center space-y-6 p-8 rounded-lg bg-white elevation-3">
            <Logo size={32} className="mx-auto" />
            <div className="space-y-3">
              <div className="w-6 h-6 border-2 border-gray-600 border-t-transparent rounded-full animate-spin mx-auto" />
              <p className="text-sm text-gray-600 font-medium">Загрузка графа НПА…</p>
            </div>
          </div>
        </div>
      )}

      {/* Graph canvas */}
      {activeData && (
        <Graph
          data={activeData}
          onNodeClick={handleNodeClick}
          selectedId={selectedNode?.id ?? null}
          highlightIds={searchHits.length > 0 ? searchHits : undefined}
          compareId={compareNode?.id ?? null}
        />
      )}

      {/* Analysis progress */}
      <AnalysisProgress state={stream} onStop={stopStream} />

      {/* Sidebar */}
      <Sidebar
        node={selectedNode}
        issues={activeData?.issues ?? []}
        stats={activeStats}
        onClose={() => setSelectedNode(null)}
        onCompareSelect={handleCompareSelect}
        compareNode={compareNode}
      />

      {/* ── Top-left brand bar ── */}
      <div className="absolute top-4 left-4 z-10">
        <div className="flex items-center h-12 rounded-lg bg-white/90 backdrop-blur-md border border-white/20 elevation-2 overflow-hidden divide-x divide-gray-100">
          {/* Brand */}
          <div className="flex items-center gap-3 px-5 h-full bg-gradient-to-r from-gray-50 to-slate-50">
            <div className="relative">
              <Logo size={20} />
              <div className="absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 bg-green-500 rounded-full border-2 border-white elevation-1" />
            </div>
          </div>

          {/* Анализ */}
          <button
            onClick={startStream}
            disabled={stream.running}
            className="h-full flex items-center gap-2 px-5 text-[11px] font-medium text-gray-600 hover:text-gray-900 hover:bg-gradient-to-r hover:from-gray-50 hover:to-slate-50 disabled:opacity-40 transition-all duration-300 relative group ripple"
          >
            {stream.running ? (
              <div className="w-4 h-4 border-2 border-gray-600 border-t-transparent rounded-full animate-spin" />
            ) : (
              <svg width="13" height="13" viewBox="0 0 12 12" fill="none" className="group-hover:rotate-90 transition-transform duration-300">
                <path d="M10.5 6A4.5 4.5 0 1 1 7.5 1.9" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round"/>
                <path d="M7.5 1v2.5H10" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" strokeLinejoin="round"/>
              </svg>
            )}
            <span className="group-hover:translate-x-0.5 transition-transform duration-300">{stream.running ? 'Анализ…' : 'Анализ'}</span>
          </button>

          {/* AI обзор */}
          <button
            onClick={fetchCorpusReview}
            disabled={corpusReviewLoading}
            className="h-full flex items-center gap-2 px-5 text-[11px] font-medium text-gray-600 hover:text-gray-900 hover:bg-gradient-to-r hover:from-gray-50 hover:to-slate-50 disabled:opacity-40 transition-all duration-300 relative group ripple"
          >
            {corpusReviewLoading ? (
              <div className="w-4 h-4 border-2 border-gray-600 border-t-transparent rounded-full animate-spin" />
            ) : (
              <span className="text-sm leading-none group-hover:scale-110 transition-transform duration-300">✦</span>
            )}
            <span className="group-hover:translate-x-0.5 transition-transform duration-300">{corpusReviewLoading ? 'AI анализ…' : 'AI обзор'}</span>
          </button>
        </div>
      </div>

      {/* ── Search bar — top center ── */}
      <div className="absolute top-4 left-1/2 -translate-x-1/2 z-20">
        <div className="flex items-center h-12 rounded-lg bg-white/90 backdrop-blur-md border border-white/20 elevation-2">
          <SearchBar onResults={setSearchHits} embedded />
          {searchHits.length > 0 && (
            <span className="mr-4 px-3 py-1.5 text-[10px] font-bold rounded-full bg-gradient-to-r from-gray-700 to-gray-800 text-white tabular-nums elevation-1">
              {searchHits.length}
            </span>
          )}
        </div>
      </div>

      {/* ── Corpus AI review panel — below topbar ── */}
      {corpusReview && !showCompare && (
        <div className="absolute top-[52px] left-4 z-20 w-80 rounded-lg bg-white border border-gray-300 shadow-lg overflow-hidden">
          <div className="h-0.5 bg-gray-800" />
          <div className="p-5 space-y-2.5">
            <div className="flex items-center justify-between">
              <span className="text-[9px] font-bold uppercase tracking-widest text-gray-700">
                {corpusReview.llm_ready ? '✦ AI-обзор корпуса (Qwen2)' : '⏳ AI-обзор'}
              </span>
              <button onClick={() => setCorpusReview(null)} className="text-gray-400 hover:text-gray-600 transition-colors">
                <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                  <path d="M1 1l8 8M9 1L1 9" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
                </svg>
              </button>
            </div>
            <p className="text-[11px] text-gray-700 leading-relaxed">{corpusReview.review}</p>
          </div>
        </div>
      )}

      {/* ── Compare panel — center canvas area ── */}
      {showCompare && (
        <ComparePanel
          nodeA={compareNode!}
          nodeB={selectedNode!}
          onClose={() => setCompareNode(null)}
        />
      )}

      {/* ── Legend — bottom center ── */}
      <div className="absolute bottom-4 left-1/2 -translate-x-1/2 z-10 pointer-events-none">
        <div className="flex items-center gap-4 px-6 py-3.5 rounded-lg bg-white/90 backdrop-blur-md border border-white/20 elevation-2 pointer-events-auto">
          {LEGEND_ITEMS.map(({ color, label }) => (
            <div key={label} className="flex items-center gap-2.5 group">
              <span className="w-3 h-3 rounded-full shrink-0 ring-2 ring-white group-hover:ring-gray-200 transition-all duration-300 elevation-1" style={{ background: color }} />
              <span className="text-[10px] text-gray-600 whitespace-nowrap group-hover:text-gray-900 transition-colors duration-300">{label}</span>
            </div>
          ))}
          <div className="w-px h-5 bg-gradient-to-b from-transparent via-gray-300 to-transparent" />
          <span className="text-[10px] text-gray-500 whitespace-nowrap">Размер — ссылки</span>
          {selectedNode && !compareNode && (
            <>
              <div className="w-px h-5 bg-gradient-to-b from-transparent via-gray-300 to-transparent" />
              <span className="text-[10px] text-gray-500 whitespace-nowrap">Ctrl+клик — сравнить</span>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
