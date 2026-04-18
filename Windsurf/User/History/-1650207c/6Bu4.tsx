import { useRef, useCallback, useEffect, useMemo, useState } from 'react';
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
  
  // Level of Detail (LOD) system
  const [zoomLevel, setZoomLevel] = useState(1);

  // Memoized filtered data for performance
  const filteredData = useMemo(() => {
    if (!data || data.nodes.length < 500) return data;
    
    // For very large graphs, show only important nodes at low zoom
    if (zoomLevel < 0.5) {
      const importantNodes = data.nodes.filter(node => 
        node.ref_count > 5 || node.issue_count > 0 || node.id === selectedId || node.id === compareId
      );
      
      const importantNodeIds = new Set(importantNodes.map(n => n.id));
      const filteredLinks = data.links.filter(link => 
        importantNodeIds.has(link.source as string) && importantNodeIds.has(link.target as string)
      );
      
      return { nodes: importantNodes, links: filteredLinks };
    }
    
    return data;
  }, [data, zoomLevel, selectedId, compareId]);

  useEffect(() => {
    const fg = graphRef.current;
    if (!fg) return;
    
    // Adaptive force settings based on graph size
    const nodeCount = filteredData.nodes.length;
    const chargeStrength = nodeCount > 1000 ? -200 : nodeCount > 500 ? -250 : -300;
    const linkDistance = nodeCount > 1000 ? 40 : nodeCount > 500 ? 50 : 60;
    
    fg.d3Force('charge')?.strength(chargeStrength);
    fg.d3Force('collision', forceCollide((node: any) =>
      Math.max(2, Math.sqrt((node.ref_count ?? 0) + 1) * 1.5) + 6
    ));
    fg.d3Force('link')?.distance(linkDistance);
    
    // Disable some forces for very large graphs
    if (nodeCount > 1000) {
      fg.d3Force('center')?.strength(0.1);
      fg.d3Force('x')?.strength(0.05);
      fg.d3Force('y')?.strength(0.05);
    }
    
    fg.d3ReheatSimulation();
    setTimeout(() => fg.zoomToFit(300, 40), 200);
  }, [filteredData]);

  // Zoom level tracking for LOD
  const handleZoom = useCallback(() => {
    if (graphRef.current) {
      const k = graphRef.current?.camera()?.z || 1;
      setZoomLevel(k);
    }
  }, []);

  // Simplified node painting with LOD
  const paintNode = useCallback(
    (node: GraphNode, ctx: CanvasRenderingContext2D, globalScale: number) => {
      const x = node.x ?? 0;
      const y = node.y ?? 0;
      const r = Math.max(2, Math.sqrt(node.ref_count + 1) * 1.5);

      const isSelected = node.id === selectedId;
      const isCompare = node.id === compareId;
      const isHit = hasHighlight && highlightIds!.includes(node.id);
      const isDimmed = (hasHighlight && !isHit) || (!isSelected && !isCompare && selectedId !== null);

      // Skip detailed rendering at low zoom - LOD system
      if (globalScale < 0.3 && !isSelected && !isCompare && !isHit) {
        ctx.beginPath();
        ctx.arc(x, y, Math.max(1, r * 0.5), 0, 2 * Math.PI);
        ctx.fillStyle = isDimmed ? '#f0f0f0' : '#d0d0d0';
        ctx.fill();
        return;
      }

      // Selection ring (simplified)
      if (isSelected || isCompare) {
        ctx.beginPath();
        ctx.arc(x, y, r + 3, 0, 2 * Math.PI);
        ctx.fillStyle = isSelected ? 'rgba(0,0,0,0.1)' : 'rgba(102,102,102,0.1)';
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

      // Minimal details at medium zoom
      if (globalScale > 0.5) {
        if (node.issue_count > 0 && !isDimmed && !node.is_amendment) {
          ctx.strokeStyle = '#666666';
          ctx.lineWidth = 0.8 / globalScale;
          ctx.stroke();
        }
      }

      // Labels only at high zoom or for selected nodes
      if ((globalScale > 1.0 && r > 4) || isSelected || isHit) {
        const label = node.title.length > 20 ? node.title.slice(0, 20) + '…' : node.title;
        const fontSize = Math.max(8, 10 / globalScale);
        ctx.font = `${fontSize}px Inter, system-ui, sans-serif`;
        ctx.fillStyle = isDimmed ? 'rgba(156,163,175,0.4)' : 'rgba(17,24,39,0.8)';
        ctx.textAlign = 'center';
        ctx.fillText(label, x, y + r + fontSize * 0.7);
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
      graphData={filteredData as any}
      nodeId="id"
      nodeLabel={nodeLabel as any}
      nodeCanvasObject={paintNode as any}
      nodeCanvasObjectMode={() => 'replace'}
      linkColor={() => 'rgba(107,114,128,0.12)'}
      linkWidth={0.6}
      linkDirectionalArrowLength={2}
      linkDirectionalArrowRelPos={1}
      linkDirectionalArrowColor={() => 'rgba(107,114,128,0.25)'}
      onNodeClick={(node: any, event: any) => onNodeClick(node as GraphNode, event)}
      onZoom={handleZoom}
      backgroundColor="#f9fafb"
      cooldownTicks={30}
      enableNodeDrag={false}
      enableZoomInteraction={true}
      enablePanInteraction={true}
      // Performance optimizations
      warmupTicks={0}
    />
  );
}
