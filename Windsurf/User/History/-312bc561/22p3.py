import networkx as nx
import matplotlib.pyplot as plt
import plotly.graph_objects as go
import plotly.express as px
from typing import List, Dict, Any, Optional, Tuple
import json
import numpy as np
from datetime import datetime
import logging

from ..core.legal_analyzer import LegalDocument, LegalNorm, AnalysisResult

logger = logging.getLogger(__name__)


class LegalGraph:
    """
    Класс для построения и визуализации графов связей между юридическими документами
    """
    
    def __init__(self):
        """Инициализация графа"""
        self.graph = nx.DiGraph()
        self.node_colors = {
            'document': '#1f77b4',      # Синий
            'norm': '#ff7f0e',          # Оранжевый
            'conflict': '#d62728',       # Красный
            'duplicate': '#9467bd',      # Фиолетовый
            'reference': '#2ca02c',      # Зеленый
            'hierarchical': '#8c564b'    # Коричневый
        }
        self.edge_colors = {
            'reference': '#2ca02c',
            'conflict': '#d62728',
            'duplicate': '#9467bd',
            'hierarchical': '#8c564b',
            'semantic': '#17becf',
            'temporal': '#bcbd22'
        }
    
    def build_graph(self, document: LegalDocument, norms: List[LegalNorm], 
                   relationships: List[Dict[str, Any]]) -> Dict[str, Any]:
        """
        Построение графа для одного документа
        
        Args:
            document: Юридический документ
            norms: Список норм
            relationships: Список связей
            
        Returns:
            Dict[str, Any]: Данные графа для визуализации
        """
        logger.info(f"Построение графа для документа: {document.title}")
        
        # Создание нового графа
        self.graph = nx.DiGraph()
        
        # Добавление узла документа
        self.graph.add_node(document.id, 
                           type='document',
                           title=document.title,
                           document_type=document.document_type,
                           adoption_date=document.adoption_date,
                           size=30)
        
        # Добавление узлов норм
        for norm in norms:
            self.graph.add_node(norm.id,
                              type='norm',
                              content=norm.content[:100] + '...',
                              norm_type=norm.norm_type,
                              number=norm.number,
                              size=20)
            
            # Связь нормы с документом
            self.graph.add_edge(document.id, norm.id,
                              type='contains',
                              weight=1.0)
        
        # Добавление связей
        for rel in relationships:
            source = rel.get('source_norm_id') or rel.get('source_document_id')
            target = rel.get('target_norm_id') or rel.get('target_document_id')
            
            if source and target and source in self.graph.nodes and target in self.graph.nodes:
                self.graph.add_edge(source, target,
                                  type=rel['relationship_type'],
                                  weight=rel.get('confidence', 0.5),
                                  evidence=rel.get('evidence', ''))
        
        # Расчет метрик графа
        metrics = self._calculate_graph_metrics()
        
        # Подготовка данных для визуализации
        viz_data = self._prepare_visualization_data()
        
        return {
            'graph_data': viz_data,
            'metrics': metrics,
            'nodes_count': self.graph.number_of_nodes(),
            'edges_count': self.graph.number_of_edges()
        }
    
    def build_multi_document_graph(self, results: List[AnalysisResult]) -> Dict[str, Any]:
        """
        Построение графа для нескольких документов
        
        Args:
            results: Результаты анализа документов
            
        Returns:
            Dict[str, Any]: Данные графа для визуализации
        """
        logger.info(f"Построение графа для {len(results)} документов")
        
        # Создание нового графа
        self.graph = nx.DiGraph()
        
        # Добавление всех документов и норм
        for result in results:
            document = result.document
            
            # Добавление узла документа
            self.graph.add_node(document.id,
                              type='document',
                              title=document.title,
                              document_type=document.document_type,
                              adoption_date=document.adoption_date,
                              size=30)
            
            # Добавление узлов норм
            for norm in result.norms:
                self.graph.add_node(norm.id,
                                  type='norm',
                                  content=norm.content[:100] + '...',
                                  norm_type=norm.norm_type,
                                  number=norm.number,
                                  document_id=document.id,
                                  size=20)
                
                # Связь нормы с документом
                self.graph.add_edge(document.id, norm.id,
                                  type='contains',
                                  weight=1.0)
            
            # Добавление связей
            for rel in result.relationships:
                source = rel.get('source_norm_id') or rel.get('source_document_id')
                target = rel.get('target_norm_id') or rel.get('target_document_id')
                
                if source and target:
                    self.graph.add_edge(source, target,
                                      type=rel['relationship_type'],
                                      weight=rel.get('confidence', 0.5),
                                      evidence=rel.get('evidence', ''))
        
        # Добавление междокументных связей (конфликты и дубликаты)
        self._add_cross_document_connections(results)
        
        # Расчет метрик графа
        metrics = self._calculate_graph_metrics()
        
        # Подготовка данных для визуализации
        viz_data = self._prepare_visualization_data()
        
        return {
            'graph_data': viz_data,
            'metrics': metrics,
            'nodes_count': self.graph.number_of_nodes(),
            'edges_count': self.graph.number_of_edges()
        }
    
    def visualize_matplotlib(self, layout: str = 'spring', figsize: Tuple[int, int] = (12, 8)) -> plt.Figure:
        """
        Визуализация графа с помощью matplotlib
        
        Args:
            layout: Тип расположения ('spring', 'circular', 'random', 'shell')
            figsize: Размер фигуры
            
        Returns:
            plt.Figure: График matplotlib
        """
        plt.figure(figsize=figsize)
        
        # Выбор расположения
        if layout == 'spring':
            pos = nx.spring_layout(self.graph, k=1, iterations=50)
        elif layout == 'circular':
            pos = nx.circular_layout(self.graph)
        elif layout == 'random':
            pos = nx.random_layout(self.graph)
        elif layout == 'shell':
            pos = nx.shell_layout(self.graph)
        else:
            pos = nx.spring_layout(self.graph)
        
        # Цвета узлов
        node_colors = [self.node_colors.get(self.graph.nodes[node]['type'], '#cccccc') 
                      for node in self.graph.nodes()]
        
        # Размеры узлов
        node_sizes = [self.graph.nodes[node].get('size', 20) * 50 
                     for node in self.graph.nodes()]
        
        # Цвета ребер
        edge_colors = [self.edge_colors.get(self.graph.edges[edge]['type'], '#cccccc') 
                      for edge in self.graph.edges()]
        
        # Ширина ребер
        edge_widths = [self.graph.edges[edge].get('weight', 0.5) * 3 
                      for edge in self.graph.edges()]
        
        # Отрисовка графа
        nx.draw(self.graph, pos,
               node_color=node_colors,
               node_size=node_sizes,
               edge_color=edge_colors,
               width=edge_widths,
               with_labels=False,
               alpha=0.8,
               arrows=True,
               arrowsize=20)
        
        # Добавление меток для документов
        document_labels = {node: self.graph.nodes[node]['title'][:20] + '...' 
                          for node in self.graph.nodes() 
                          if self.graph.nodes[node]['type'] == 'document'}
        
        nx.draw_networkx_labels(self.graph, pos, labels=document_labels, font_size=8)
        
        plt.title("Граф связей юридических документов", fontsize=16)
        plt.axis('off')
        
        # Добавление легенды
        self._add_matplotlib_legend()
        
        return plt.gcf()
    
    def visualize_plotly(self, layout: str = 'spring') -> go.Figure:
        """
        Визуализация графа с помощью plotly (интерактивная)
        
        Args:
            layout: Тип расположения
            
        Returns:
            go.Figure: Интерактивный график plotly
        """
        # Выбор расположения
        if layout == 'spring':
            pos = nx.spring_layout(self.graph, k=1, iterations=50)
        elif layout == 'circular':
            pos = nx.circular_layout(self.graph)
        else:
            pos = nx.spring_layout(self.graph)
        
        # Подготовка данных для ребер
        edge_x = []
        edge_y = []
        edge_info = []
        
        for edge in self.graph.edges():
            x0, y0 = pos[edge[0]]
            x1, y1 = pos[edge[1]]
            edge_x.extend([x0, x1, None])
            edge_y.extend([y0, y1, None])
            
            edge_data = self.graph.edges[edge]
            edge_info.append(f"Тип: {edge_data['type']}<br>Вес: {edge_data.get('weight', 0):.2f}")
        
        # Создание линий ребер
        edge_trace = go.Scatter(
            x=edge_x, y=edge_y,
            line=dict(width=1, color='#888'),
            hoverinfo='none',
            mode='lines'
        )
        
        # Подготовка данных для узлов
        node_x = []
        node_y = []
        node_text = []
        node_info = []
        node_colors = []
        node_sizes = []
        
        for node in self.graph.nodes():
            x, y = pos[node]
            node_x.append(x)
            node_y.append(y)
            
            node_data = self.graph.nodes[node]
            node_type = node_data['type']
            
            if node_type == 'document':
                node_text.append(node_data['title'][:30] + '...')
                node_info.append(f"Документ: {node_data['title']}<br>Тип: {node_data['document_type']}")
            else:
                node_text.append(f"{node_data['norm_type']} {node_data['number']}")
                node_info.append(f"Норма: {node_data['norm_type']} {node_data['number']}<br>{node_data['content']}")
            
            node_colors.append(self.node_colors.get(node_type, '#cccccc'))
            node_sizes.append(node_data.get('size', 20) * 3)
        
        # Создание узлов
        node_trace = go.Scatter(
            x=node_x, y=node_y,
            mode='markers+text',
            hoverinfo='text',
            text=node_text,
            textposition="middle center",
            hovertext=node_info,
            marker=dict(
                size=node_sizes,
                color=node_colors,
                line=dict(width=2, color='white')
            )
        )
        
        # Создание фигуры
        fig = go.Figure(data=[edge_trace, node_trace],
                       layout=go.Layout(
                           title='Интерактивный граф связей юридических документов',
                           titlefont_size=16,
                           showlegend=False,
                           hovermode='closest',
                           margin=dict(b=20, l=5, r=5, t=40),
                           annotations=[dict(
                               text="",
                               showarrow=False,
                               xref="paper", yref="paper",
                               x=0.005, y=-0.002,
                               xanchor='left', yanchor='bottom',
                               font=dict(color="#888", size=12)
                           )],
                           xaxis=dict(showgrid=False, zeroline=False, showticklabels=False),
                           yaxis=dict(showgrid=False, zeroline=False, showticklabels=False))
                       )
        
        return fig
    
    def export_graph(self, filename: str, format: str = 'json') -> bool:
        """
        Экспорт графа в файл
        
        Args:
            filename: Имя файла
            format: Формат ('json', 'gexf', 'graphml')
            
        Returns:
            bool: Успешность экспорта
        """
        try:
            if format == 'json':
                data = nx.node_link_data(self.graph)
                with open(filename, 'w', encoding='utf-8') as f:
                    json.dump(data, f, ensure_ascii=False, indent=2, default=str)
            elif format == 'gexf':
                nx.write_gexf(self.graph, filename)
            elif format == 'graphml':
                nx.write_graphml(self.graph, filename)
            else:
                logger.error(f"Неподдерживаемый формат: {format}")
                return False
            
            logger.info(f"Граф экспортирован в файл: {filename}")
            return True
            
        except Exception as e:
            logger.error(f"Ошибка при экспорте графа: {e}")
            return False
    
    def _calculate_graph_metrics(self) -> Dict[str, Any]:
        """Расчет метрик графа"""
        if self.graph.number_of_nodes() == 0:
            return {}
        
        metrics = {
            'nodes_count': self.graph.number_of_nodes(),
            'edges_count': self.graph.number_of_edges(),
            'density': nx.density(self.graph),
            'is_connected': nx.is_weakly_connected(self.graph),
            'components': nx.number_weakly_connected_components(self.graph)
        }
        
        # Центральность узлов
        if self.graph.number_of_nodes() > 1:
            degree_centrality = nx.degree_centrality(self.graph)
            betweenness_centrality = nx.betweenness_centrality(self.graph)
            
            metrics['most_central_node'] = max(degree_centrality, key=degree_centrality.get)
            metrics['highest_betweenness'] = max(betweenness_centrality, key=betweenness_centrality.get)
            metrics['average_degree'] = sum(dict(self.graph.degree()).values()) / self.graph.number_of_nodes()
        
        # Метрики по типам узлов
        node_types = {}
        for node in self.graph.nodes():
            node_type = self.graph.nodes[node]['type']
            node_types[node_type] = node_types.get(node_type, 0) + 1
        
        metrics['node_types'] = node_types
        
        # Метрики по типам ребер
        edge_types = {}
        for edge in self.graph.edges():
            edge_type = self.graph.edges[edge]['type']
            edge_types[edge_type] = edge_types.get(edge_type, 0) + 1
        
        metrics['edge_types'] = edge_types
        
        return metrics
    
    def _prepare_visualization_data(self) -> Dict[str, Any]:
        """Подготовка данных для визуализации"""
        # Позиции узлов (используем spring layout)
        pos = nx.spring_layout(self.graph, k=1, iterations=50)
        
        # Данные узлов
        nodes = []
        for node in self.graph.nodes():
            node_data = self.graph.nodes[node]
            nodes.append({
                'id': node,
                'type': node_data['type'],
                'x': pos[node][0],
                'y': pos[node][1],
                'size': node_data.get('size', 20),
                'color': self.node_colors.get(node_data['type'], '#cccccc'),
                'label': node_data.get('title', node_data.get('content', ''))[:50],
                'metadata': {k: v for k, v in node_data.items() if k not in ['x', 'y', 'size', 'color']}
            })
        
        # Данные ребер
        edges = []
        for edge in self.graph.edges():
            edge_data = self.graph.edges[edge]
            edges.append({
                'source': edge[0],
                'target': edge[1],
                'type': edge_data['type'],
                'weight': edge_data.get('weight', 0.5),
                'color': self.edge_colors.get(edge_data['type'], '#cccccc'),
                'metadata': {k: v for k, v in edge_data.items() if k not in ['weight', 'color']}
            })
        
        return {
            'nodes': nodes,
            'edges': edges,
            'layout': 'spring'
        }
    
    def _add_cross_document_connections(self, results: List[AnalysisResult]):
        """Добавление междокументных связей"""
        # Создание словаря норм
        all_norms = {}
        for result in results:
            for norm in result.norms:
                all_norms[norm.id] = norm
        
        # Добавление связей конфликтов
        for result in results:
            for conflict in result.conflicts:
                norm1_id = conflict.get('norm1_id')
                norm2_id = conflict.get('norm2_id')
                
                if norm1_id in all_norms and norm2_id in all_norms:
                    self.graph.add_edge(norm1_id, norm2_id,
                                      type='conflict',
                                      weight=conflict.get('confidence', 0.5),
                                      evidence=conflict.get('evidence', ''))
        
        # Добавление связей дубликатов
        for result in results:
            for duplicate in result.duplicates:
                norm1_id = duplicate.get('norm1_id')
                norm2_id = duplicate.get('norm2_id')
                
                if norm1_id in all_norms and norm2_id in all_norms:
                    self.graph.add_edge(norm1_id, norm2_id,
                                      type='duplicate',
                                      weight=duplicate.get('similarity_score', 0.5),
                                      evidence=duplicate.get('evidence', ''))
    
    def _add_matplotlib_legend(self):
        """Добавление легенды для matplotlib"""
        from matplotlib.patches import Patch
        
        legend_elements = [
            Patch(facecolor=self.node_colors['document'], label='Документ'),
            Patch(facecolor=self.node_colors['norm'], label='Норма'),
            Patch(facecolor=self.edge_colors['reference'], label='Ссылка'),
            Patch(facecolor=self.edge_colors['conflict'], label='Конфликт'),
            Patch(facecolor=self.edge_colors['duplicate'], label='Дубликат')
        ]
        
        plt.legend(handles=legend_elements, loc='upper right')
    
    def find_shortest_path(self, source: str, target: str) -> Optional[List[str]]:
        """
        Поиск кратчайшего пути между узлами
        
        Args:
            source: Исходный узел
            target: Целевой узел
            
        Returns:
            Optional[List[str]]: Путь или None если путь не найден
        """
        try:
            return nx.shortest_path(self.graph, source, target)
        except nx.NetworkXNoPath:
            return None
    
    def get_node_neighbors(self, node_id: str, depth: int = 1) -> List[str]:
        """
        Получение соседей узла
        
        Args:
            node_id: ID узла
            depth: Глубина поиска
            
        Returns:
            List[str]: Список соседних узлов
        """
        if node_id not in self.graph:
            return []
        
        neighbors = set()
        current_level = {node_id}
        
        for _ in range(depth):
            next_level = set()
            for node in current_level:
                next_level.update(self.graph.neighbors(node))
                next_level.update(self.graph.predecessors(node))
            neighbors.update(next_level)
            current_level = next_level - neighbors
        
        neighbors.discard(node_id)  # Удаляем исходный узел
        return list(neighbors)
    
    def analyze_graph_clusters(self) -> Dict[str, Any]:
        """
        Анализ кластеров в графе
        
        Returns:
            Dict[str, Any]: Информация о кластерах
        """
        if self.graph.number_of_nodes() == 0:
            return {}
        
        # Поиск сообществ с помощью алгоритма Лувена
        try:
            communities = nx.community.louvain_communities(self.graph)
        except:
            # Если алгоритм недоступен, используем простые компоненты связности
            communities = list(nx.weakly_connected_components(self.graph))
        
        cluster_info = {
            'num_clusters': len(communities),
            'clusters': []
        }
        
        for i, community in enumerate(communities):
            cluster_data = {
                'id': i,
                'size': len(community),
                'nodes': list(community),
                'node_types': {}
            }
            
            # Анализ типов узлов в кластере
            for node in community:
                if node in self.graph.nodes:
                    node_type = self.graph.nodes[node]['type']
                    cluster_data['node_types'][node_type] = cluster_data['node_types'].get(node_type, 0) + 1
            
            cluster_info['clusters'].append(cluster_data)
        
        return cluster_info
