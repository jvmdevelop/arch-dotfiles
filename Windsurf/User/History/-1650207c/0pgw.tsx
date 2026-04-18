import { useRef, useCallback, useEffect } from 'react';
// @ts-ignore — d3-force-3d is a transitive dep without typings
import { forceCollide } from 'd3-force-3d';
import ForceGraph2D from 'react-force-graph-2d';
import type { GraphData, GraphNode } from '../types';

interface GraphProps {
  data: GraphData;
  onNodeClick: (node: GraphNode, event?: MouseEvent) => void;
  selectedId: string | null;
  compareId: string | null;
  highlightIds?: string[];
}

const STATUS_COLOR: Record<string, string> = {
  active: '#000000',
  outdated: '#666666',
  unknown: '#999999',
};

// Dimmed versions for light background
const STATUS_DIM: Record<string, string> = {
  active: '#e5e5e5',
  outdated: '#d4d4d4',
  unknown: '#f5f5f5',
};

export function Graph({ data, onNodeClick, selectedId, compareId, highlightIds }: GraphProps) {
  const graphRef = useRef<any>(null);
  const hasHighlight = highlightIds && highlightIds.length > 0;

  useEffect(() => {
    const fg = graphRef.current;
    if (!fg) return;
    
    // Optimized force settings for large graphs
    fg.d3Force('charge')?.strength(-300); // Reduced from -400
    fg.d3Force('collision', forceCollide((node: any) =>
      Math.max(3, Math.sqrt((node.ref_count ?? 0) + 1) * 2) + 8 // Smaller collision radius
    ));
    fg.d3Force('link')?.distance(60); // Reduced from 80
    
    // Faster cooldown for large graphs
    fg.d3ReheatSimulation();
    setTimeout(() => fg.zoomToFit(400, 60), 300); // Faster zoom
  }, [data]);

  const paintNode = useCallback(
    (node: GraphNode, ctx: CanvasRenderingContext2D, globalScale: number) => {
      const x = node.x ?? 0;
      const y = node.y ?? 0;
      const r = Math.max(3, Math.sqrt(node.ref_count + 1) * 2); // Smaller base radius

      const isSelected = node.id === selectedId;
      const isCompare = node.id === compareId;
      const isHit = hasHighlight && highlightIds!.includes(node.id);
      const isDimmed = (hasHighlight && !isHit) || (!isSelected && !isCompare && selectedId !== null);

      // Simplified selection rings for performance
      if (isSelected) {
        ctx.beginPath();
        ctx.arc(x, y, r + 4, 0, 2 * Math.PI); // Smaller ring
        ctx.fillStyle = 'rgba(0,0,0,0.15)';
        ctx.fill();
      }
      if (isCompare) {
        ctx.beginPath();
        ctx.arc(x, y, r + 4, 0, 2 * Math.PI);
        ctx.fillStyle = 'rgba(102,102,102,0.15)';
        ctx.fill();
      }
      if (isHit) {
        ctx.beginPath();
        ctx.arc(x, y, r + 3, 0, 2 * Math.PI);
        ctx.fillStyle = 'rgba(153,153,153,0.15)';
        ctx.fill();
      }

      // Node body
      ctx.beginPath();
      ctx.arc(x, y, r, 0, 2 * Math.PI);
      if (node.is_amendment && !isDimmed) {
        ctx.fillStyle = '#333333';
      } else {
        ctx.fillStyle = isDimmed
          ? STATUS_DIM[node.status] ?? '#f5f5f5'
          : STATUS_COLOR[node.status] ?? '#999999';
      }
      ctx.fill();

      // Simplified amendment ring
      if (node.is_amendment && !isDimmed && globalScale > 0.5) { // Only show when zoomed in
        ctx.save();
        ctx.setLineDash([2 / globalScale, 2 / globalScale]);
        ctx.beginPath();
        ctx.arc(x, y, r + 2, 0, 2 * Math.PI);
        ctx.strokeStyle = 'rgba(51,51,51,0.4)';
        ctx.lineWidth = 1 / globalScale;
        ctx.stroke();
        ctx.restore();
      }

      // Issue ring - simplified
      if (node.issue_count > 0 && !isDimmed && !node.is_amendment) {
        ctx.strokeStyle = '#666666';
        ctx.lineWidth = 1 / globalScale;
        ctx.stroke();
      }

      // Selection border
      if (isSelected || isCompare) {
        ctx.strokeStyle = isCompare ? '#666666' : '#000000';
        ctx.lineWidth = 2 / globalScale;
        ctx.stroke();
      }

      // Labels only for larger nodes or when selected/zoomed
      if ((r > 5 && globalScale > 0.8) || isSelected || isHit) {
        const label = node.title.length > 25 ? node.title.slice(0, 25) + '…' : node.title; // Shorter labels
        const fontSize = Math.max(9, 11 / globalScale);
        ctx.font = `${fontSize}px Inter, system-ui, sans-serif`;
        ctx.fillStyle = isDimmed ? 'rgba(156,163,175,0.5)' : 'rgba(17,24,39,0.85)';
        ctx.textAlign = 'center';
        ctx.fillText(label, x, y + r + fontSize * 0.8);
      }
    },
    [selectedId, compareId, highlightIds, hasHighlight],
  );

  const nodeLabel = useCallback((node: GraphNode) => {
    const issues = node.issue_count > 0 ? ` | ${node.issue_count} проблем` : '';
    return `${node.title}${issues}`;
  }, []);

  return (
    <ForceGraph2D
      ref={graphRef}
      graphData={data as any}
      nodeId="id"
      nodeLabel={nodeLabel as any}
      nodeCanvasObject={paintNode as any}
      nodeCanvasObjectMode={() => 'replace'}
      linkColor={() => 'rgba(107,114,128,0.15)'} // More transparent
      linkWidth={0.8} // Thinner links
      linkDirectionalArrowLength={3} // Smaller arrows
      linkDirectionalArrowRelPos={1}
      linkDirectionalArrowColor={() => 'rgba(107,114,128,0.3)'}
      onNodeClick={(node: any, event: any) => onNodeClick(node as GraphNode, event)}
      backgroundColor="#f9fafb"
      cooldownTicks={50} // Faster cooldown
      enableNodeDrag={false} // Disable dragging for performance
      enableZoomInteraction={true}
      enablePanInteraction={true}
    />
  );
}
