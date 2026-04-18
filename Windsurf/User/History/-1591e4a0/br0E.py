

from typing import List, Dict, Any, Optional, Tuple
import re
import spacy
from datetime import datetime
import logging

from ..core.legal_analyzer import LegalNorm

logger = logging.getLogger(__name__)


class LegalExplainer:
    
    def __init__(self):
        try:
            self.nlp = spacy.load("ru_core_news_lg")
        except OSError:
            self.nlp = spacy.load("xx_ent_wiki_sm")
            logger.warning("Используется многоязычная модель вместо русской")
        
        self.explanation_templates = {
            'conflict': {
                'prohibition_permission': "Обнаружено противоречие: норма {norm1} {action1}, в то время как норма {norm2} {action2}. Это создает правовую неопределенность.",
                'obligation_negation': "Норма {norm1} устанавливает обязательство {obligation1}, но норма {norm2} отрицает его {negation2}.",
                'contradictory_conditions': "Условия в нормах противоречивы: {norm1} применяется {condition1}, а {norm2} - {condition2}.",
                'temporal_conflicts': "Временные требования противоречивы: {norm1} требует {time1}, а {norm2} - {time2}.",
                'quantitative_conflicts': "Количественные показатели противоречивы: {norm1} устанавливает {quantity1}, а {norm2} - {quantity2}."
            },
            'duplicate': {
                'exact_duplicate': "Нормы {norm1} и {norm2} полностью идентичны. Это является прямым дублированием.",
                'near_duplicate': "Нормы {norm1} и {norm2} очень похожи (схожесть {similarity:.1%}). Рекомендуется объединить или уточнить различия.",
                'semantic_duplicate': "Нормы {norm1} и {norm2} имеют одинаковое смысловое содержание, но разную формулировку.",
                'partial_duplicate': "Нормы {norm1} и {norm2} содержат общие фрагменты: {common_fragments}.",
                'structural_duplicate': "Нормы {norm1} и {norm2} имеют одинаковую структуру, что может указывать на дублирование."
            },
            'relationship': {
                'reference': "Норма {source} ссылается на {target}, что указывает на прямую связь между документами.",
                'hierarchical': "Обнаружена иерархическая связь: {source} подчиняется {target} в соответствии с юридической иерархией.",
                'semantic': "Нормы {source} и {target} семантически связаны (схожесть {similarity:.1%}).",
                'temporal': "Обнаружена временная связь между нормами {source} и {target}."
            }
        }
        
        self.explanation_keywords = {
            'conflict': ['противоречие', 'конфликт', 'несоответствие', 'нарушение'],
            'duplicate': ['дублирование', 'повторение', 'идентичность', 'схожесть'],
            'relationship': ['связь', 'зависимость', 'иерархия', 'ссылка'],
            'recommendation': ['рекомендуется', 'следует', 'необходимо', 'целесообразно']
        }
    
    def generate_explanations(self, conflicts: List[Dict[str, Any]], 
                            duplicates: List[Dict[str, Any]],
                            relationships: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """
        Генерация объяснений для всех типов проблем
        
        Args:
            conflicts: Список конфликтов
            duplicates: Список дубликатов
            relationships: Список связей
            
        Returns:
            List[Dict[str, Any]]: Список объяснений
        """
        logger.info("Генерация объяснений для результатов анализа")
        
        explanations = []
        
        conflict_explanations = self._explain_conflicts(conflicts)
        explanations.extend(conflict_explanations)
        
        duplicate_explanations = self._explain_duplicates(duplicates)
        explanations.extend(duplicate_explanations)
        
        relationship_explanations = self._explain_relationships(relationships)
        explanations.extend(relationship_explanations)
        
        recommendations = self._generate_recommendations(conflicts, duplicates, relationships)
        explanations.extend(recommendations)
        
        logger.info(f"Сгенерировано {len(explanations)} объяснений")
        return explanations
    
    def explain_specific_issue(self, issue_type: str, issue_data: Dict[str, Any], 
                             norms_dict: Dict[str, LegalNorm]) -> Dict[str, Any]:
        """
        Генерация объяснения для конкретной проблемы
        
        Args:
            issue_type: Тип проблемы ('conflict', 'duplicate', 'relationship')
            issue_data: Данные о проблеме
            norms_dict: Словарь норм
            
        Returns:
            Dict[str, Any]: Объяснение проблемы
        """
        if issue_type == 'conflict':
            return self._explain_single_conflict(issue_data, norms_dict)
        elif issue_type == 'duplicate':
            return self._explain_single_duplicate(issue_data, norms_dict)
        elif issue_type == 'relationship':
            return self._explain_single_relationship(issue_data, norms_dict)
        else:
            return {
                'type': 'unknown',
                'explanation': f'Неизвестный тип проблемы: {issue_type}',
                'confidence': 0.0,
                'recommendations': []
            }
    
    def _explain_conflicts(self, conflicts: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Генерация объяснений для конфликтов"""
        explanations = []
        
        for conflict in conflicts:
            explanation = {
                'type': 'conflict',
                'subtype': conflict.get('type', 'unknown'),
                'norm1_id': conflict.get('norm1_id'),
                'norm2_id': conflict.get('norm2_id'),
                'confidence': conflict.get('confidence', 0.0),
                'explanation': self._generate_conflict_explanation(conflict),
                'evidence': conflict.get('evidence', ''),
                'recommendations': self._generate_conflict_recommendations(conflict),
                'severity': self._assess_conflict_severity(conflict)
            }
            explanations.append(explanation)
        
        return explanations
    
    def _explain_duplicates(self, duplicates: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Генерация объяснений для дубликатов"""
        explanations = []
        
        for duplicate in duplicates:
            explanation = {
                'type': 'duplicate',
                'subtype': duplicate.get('type', 'unknown'),
                'norm1_id': duplicate.get('norm1_id'),
                'norm2_id': duplicate.get('norm2_id'),
                'similarity_score': duplicate.get('similarity_score', 0.0),
                'explanation': self._generate_duplicate_explanation(duplicate),
                'evidence': duplicate.get('evidence', ''),
                'recommendations': self._generate_duplicate_recommendations(duplicate),
                'priority': self._assess_duplicate_priority(duplicate)
            }
            explanations.append(explanation)
        
        return explanations
    
    def _explain_relationships(self, relationships: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        explanations = []
        
        important_relationships = [rel for rel in relationships 
                                 if rel.get('confidence', 0) > 0.7]
        
        for relationship in important_relationships:
            explanation = {
                'type': 'relationship',
                'subtype': relationship.get('relationship_type', 'unknown'),
                'source_id': relationship.get('source_norm_id') or relationship.get('source_document_id'),
                'target_id': relationship.get('target_norm_id') or relationship.get('target_document_id'),
                'confidence': relationship.get('confidence', 0.0),
                'explanation': self._generate_relationship_explanation(relationship),
                'evidence': relationship.get('evidence', ''),
                'importance': self._assess_relationship_importance(relationship)
            }
            explanations.append(explanation)
        
        return explanations
    
    def _generate_conflict_explanation(self, conflict: Dict[str, Any]) -> str:
        conflict_type = conflict.get('type', 'unknown')
        template = self.explanation_templates['conflict'].get(conflict_type)
        
        if template:
            return template.format(
                norm1=conflict.get('norm1_id', 'N/A'),
                norm2=conflict.get('norm2_id', 'N/A'),
                action1=conflict.get('norm1_fragment', 'действие 1'),
                action2=conflict.get('norm2_fragment', 'действие 2'),
                obligation1=conflict.get('norm1_fragment', 'обязательство 1'),
                negation2=conflict.get('norm2_fragment', 'отрицание 2'),
                condition1=conflict.get('norm1_fragment', 'условие 1'),
                condition2=conflict.get('norm2_fragment', 'условие 2'),
                time1=conflict.get('norm1_fragment', 'временное требование 1'),
                time2=conflict.get('norm2_fragment', 'временное требование 2'),
                quantity1=conflict.get('norm1_fragment', 'количество 1'),
                quantity2=conflict.get('norm2_fragment', 'количество 2')
            )
        else:
            return f"Обнаружен конфликт типа '{conflict_type}' между нормами {conflict.get('norm1_id')} и {conflict.get('norm2_id')}. Уверенность: {conflict.get('confidence', 0):.2f}."
    
    def _generate_duplicate_explanation(self, duplicate: Dict[str, Any]) -> str:
        duplicate_type = duplicate.get('type', 'unknown')
        template = self.explanation_templates['duplicate'].get(duplicate_type)
        
        if template:
            similarity = duplicate.get('similarity_score', 0.0)
            common_fragments = duplicate.get('common_substrings', [])
            
            return template.format(
                norm1=duplicate.get('norm1_id', 'N/A'),
                norm2=duplicate.get('norm2_id', 'N/A'),
                similarity=similarity,
                common_fragments=', '.join(common_fragments[:2]) if common_fragments else 'общие фрагменты'
            )
        else:
            return f"Обнаружено дублирование типа '{duplicate_type}' между нормами {duplicate.get('norm1_id')} и {duplicate.get('norm2_id')}. Схожесть: {duplicate.get('similarity_score', 0):.2f}."
    
    def _generate_relationship_explanation(self, relationship: Dict[str, Any]) -> str:
        rel_type = relationship.get('relationship_type', 'unknown')
        template = self.explanation_templates['relationship'].get(rel_type)
        
        if template:
            return template.format(
                source=relationship.get('source_norm_id') or relationship.get('source_document_id', 'N/A'),
                target=relationship.get('target_norm_id') or relationship.get('target_document_id', 'N/A'),
                similarity=relationship.get('confidence', 0.0)
            )
        else:
            return f"Обнаружена связь типа '{rel_type}' между {relationship.get('source_norm_id') or relationship.get('source_document_id')} и {relationship.get('target_norm_id') or relationship.get('target_document_id')}."
    
    def _generate_recommendations(self, conflicts: List[Dict[str, Any]], 
                                duplicates: List[Dict[str, Any]],
                                relationships: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        recommendations = []
        
        if conflicts:
            high_severity_conflicts = [c for c in conflicts if c.get('confidence', 0) > 0.8]
            if highacheverity_conflicts:
                recommendations.append({
                    'type': 'recommendation',
                    'category': 'conflict_resolution',
                    'priority': 'high',
                    'title': 'Урегулирование высокоприоритетных конфликтов',
                    'description': f'Обнаружено {len(high_severity_conflicts)} конфликтов с высокой степенью уверенности. Рекомендуется срочно провести юридическую экспертизу для их разрешения.',
                    'actions': [
                        'Провести анализ юридической иерархии',
                        'Определить норму с большей юридической силой',
                        'Подготовить предложения по изменению или отмене противоречивых норм'
                    ]
                })
        
    
        if duplicates:
            exact_duplicates = [d for d in duplicates if d.get('type') == 'exact_duplicate']
            if exact_duplicates:
                recommendations.append({
                    'type': 'recommendation',
                    'category': 'duplicate_elimination',
                    'priority': 'medium',
                    'title': 'Устранение точных дубликатов',
                    'description': f'Обнаружено {len(exact_duplicates)} точных дубликатов. Рекомендуется объединить или удалить повторяющиеся нормы.',
                    'actions': [
                        'Определить основную норму',
                        'Удалить или объединить дублирующие нормы',
                        'Обновить ссылки в других документах'
                    ]
                })
        
        total_issues = len(conflicts) + len(duplicates)
        if total_issues > 10:
            recommendations.append({
                'type': 'recommendation',
                'category': 'optimization',
                'priority': 'low',
                'title': 'Оптимизация нормативной базы',
                'description': f'Обнаружено большое количество проблем ({total_issues}). Рекомендуется провести комплексную ревизию нормативной базы.',
                'actions': [
                    'Создать план по систематизации законодательства',
                    'Внедрить систему мониторинга нормативных актов',
                    'Разработать методику предотвращения дублирования'
                ]
            })
        
        return recommendations
    
    def _generate_conflict_recommendations(self, conflict: Dict[str, Any]) -> List[str]:
        recommendations = []
        
        conflict_type = conflict.get('type', 'unknown')
        confidence = conflict.get('confidence', 0.0)
        
        if confidence > 0.8:
            recommendations.append("Срочно провести юридическую экспертизу для разрешения конфликта")
        
        if conflict_type == 'prohibition_permission':
            recommendations.extend([
                "Определить приоритет нормы (запрет или разрешение)",
                "Уточнить условия применения каждой нормы",
                "Рассмотреть возможность введения исключений"
            ])
        elif conflict_type == 'obligation_negation':
            recommendations.extend([
                "Определить, какая норма имеет большую юридическую силу",
                "Проверить актуальность обеих норм",
                "Подготовить изменения для устранения противоречия"
            ])
        elif conflict_type == 'temporal_conflicts':
            recommendations.extend([
                "Установить четкие временные рамки",
                "Определить приоритет временных требований",
                "Согласовать сроки выполнения обязательств"
            ])
        
        return recommendations
    
    def _generate_duplicate_recommendations(self, duplicate: Dict[str, Any]) -> List[str]:
        recommendations = []
        
        duplicate_type = duplicate.get('type', 'unknown')
        similarity = duplicate.get('similarity_score', 0.0)
        
        if duplicate_type == 'exact_duplicate':
            recommendations.extend([
                "Удалить одну из дублирующих норм",
                "Обновить все ссылки на удаляемую норму",
                "Проверить связанные документы на наличие аналогичных дубликатов"
            ])
        elif duplicate_type == 'near_duplicate':
            recommendations.extend([
                "Объединить похожие нормы в одну",
                "Уточнить различия между нормами",
                "Рассмотреть возможность дифференциации применения"
            ])
        elif duplicate_type == 'semantic_duplicate':
            recommendations.extend([
                "Гармонизировать формулировки норм",
                "Установить единую терминологию",
                "Разработать единые стандарты изложения норм"
            ])
        
        if similarity > 0.9:
            recommendations.append("Приоритет: немедленное устранение дублирования")
        elif similarity > 0.7:
            recommendations.append("Приоритет: плановое устранение дублирования")
        
        return recommendations
    
    def _assess_conflict_severity(self, conflict: Dict[str, Any]) -> str:
        confidence = conflict.get('confidence', 0.0)
        conflict_type = conflict.get('type', 'unknown')
        
        if confidence > 0.9 or conflict_type in ['prohibition_permission', 'obligation_negation']:
            return 'critical'
        elif confidence > 0.7:
            return 'high'
        elif confidence > 0.5:
            return 'medium'
        else:
            return 'low'
    
    def _assess_duplicate_priority(self, duplicate: Dict[str, Any]) -> str:
    
        similarity = duplicate.get('similarity_score', 0.0)
        duplicate_type = duplicate.get('type', 'unknown')
        
        if duplicate_type == 'exact_duplicate' or similarity > 0.95:
            return 'high'
        elif similarity > 0.8:
            return 'medium'
        else:
            return 'low'
    
    def _assess_relationship_importance(self, relationship: Dict[str, Any]) -> str:
        confidence = relationship.get('confidence', 0.0)
        rel_type = relationship.get('relationship_type', 'unknown')
        
        if rel_type in ['hierarchical', 'reference'] and confidence > 0.8:
            return 'critical'
        elif confidence > 0.7:
            return 'high'
        elif confidence > 0.5:
            return 'medium'
        else:
            return 'low'
    
    def _explain_single_conflict(self, conflict: Dict[str, Any], 
                               norms_dict: Dict[str, LegalNorm]) -> Dict[str, Any]:
        norm1 = norms_dict.get(conflict.get('norm1_id'))
        norm2 = norms_dict.get(conflict.get('norm2_id'))
        
        explanation = self._generate_conflict_explanation(conflict)
        recommendations = self._generate_conflict_recommendations(conflict)
        
        context = {
            'norm1_content': norm1.content[:200] + '...' if norm1 else 'Не найдена',
            'norm2_content': norm2.content[:200] + '...' if norm2 else 'Не найдена',
            'norm1_type': norm1.norm_type if norm1 else 'unknown',
            'norm2_type': norm2.norm_type if norm2 else 'unknown'
        }
        
        return {
            'type': 'conflict',
            'explanation': explanation,
            'recommendations': recommendations,
            'context': context,
            'severity': self._assess_conflict_severity(conflict)
        }
    
    def _explain_single_duplicate(self, duplicate: Dict[str, Any], 
                                norms_dict: Dict[str, LegalNorm]) -> Dict[str, Any]:
        norm1 = norms_dict.get(duplicate.get('norm1_id'))
        norm2 = norms_dict.get(duplicate.get('norm2_id'))
        
        explanation = self._generate_duplicate_explanation(duplicate)
        recommendations = self._generate_duplicate_recommendations(duplicate)
        
        context = {
            'norm1_content': norm1.content[:200] + '...' if norm1 else 'Не найдена',
            'norm2_content': norm2.content[:200] + '...' if norm2 else 'Не найдена',
            'similarity_score': duplicate.get('similarity_score', 0.0),
            'common_fragments': duplicate.get('common_substrings', [])
        }
        
        return {
            'type': 'duplicate',
            'explanation': explanation,
            'recommendations': recommendations,
            'context': context,
            'priority': self._assess_duplicate_priority(duplicate)
        }
    
    def _explain_single_relationship(self, relationship: Dict[str, Any], 
                                   norms_dict: Dict[str, LegalNorm]) -> Dict[str, Any]:
        source_id = relationship.get('source_norm_id') or relationship.get('source_document_id')
        target_id = relationship.get('target_norm_id') or relationship.get('target_document_id')
        
        source_norm = norms_dict.get(source_id)
        target_norm = norms_dict.get(target_id)
        
        explanation = self._generate_relationship_explanation(relationship)
        
        context = {
            'source_content': source_norm.content[:200] + '...' if source_norm else 'Документ',
            'target_content': target_norm.content[:200] + '...' if target_norm else 'Документ',
            'relationship_type': relationship.get('relationship_type', 'unknown'),
            'confidence': relationship.get('confidence', 0.0)
        }
        
        return {
            'type': 'relationship',
            'explanation': explanation,
            'context': context,
            'importance': self._assess_relationship_importance(relationship)
        }
    
    def generate_summary_report(self, explanations: List[Dict[str, Any]]) -> Dict[str, Any]:

        by_type = {}
        for explanation in explanations:
            exp_type = explanation.get('type', 'unknown')
            if exp_type not in by_type:
                by_type[exp_type] = []
            by_type[exp_type].append(explanation)
        
    
        stats = {
            'total_explanations': len(explanations),
            'by_type': {k: len(v) for k, v in by_type.items()},
            'high_priority': len([e for e in explanations if e.get('severity') == 'critical' or e.get('priority') == 'high']),
            'recommendations': len([e for e in explanations if e.get('type') == 'recommendation'])
        }
        
        key_issues = []
        for exp_type, explanations_list in by_type.items():
            if exp_type in ['conflict', 'duplicate']:
                sorted_explanations = sorted(explanations_list, 
                                           key=lambda x: x.get('confidence', x.get('similarity_score', 0)), 
                                           reverse=True)
                key_issues.extend(sorted_explanations[:3])
        
        return {
            'statistics': stats,
            'key_issues': key_issues,
            'recommendations': by_type.get('recommendation', []),
            'generated_at': datetime.now().isoformat()
        }
