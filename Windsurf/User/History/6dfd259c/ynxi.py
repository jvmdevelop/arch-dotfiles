from flask import Flask, request, jsonify
from sentence_transformers import SentenceTransformer
import numpy as np
import torch

app = Flask(__name__)

model = SentenceTransformer('all-MiniLM-L6-v2')

@app.route('/health', methods=['GET'])
def health():
    return jsonify({"status": "healthy", "model": "all-MiniLM-L6-v2"})

@app.route('/encode', methods=['POST'])
def encode():
    try:
        data = request.json
        sentences = data.get('sentences', [])
        
        if not sentences:
            return jsonify({"error": "No sentences provided"}), 400
        
        # Get embeddings
        embeddings = model.encode(sentences)
        
        # Convert to list if it's a numpy array
        if hasattr(embeddings, 'tolist'):
            embeddings = embeddings.tolist()
        
        return jsonify({"embeddings": embeddings})
        
    except Exception as e:
        return jsonify({"error": str(e)}), 500

@app.route('/similarity', methods=['POST'])
def similarity():
    try:
        data = request.json
        pairs = data.get('pairs', [])
        
        if not pairs:
            return jsonify({"error": "No pairs provided"}), 400
        
        # Extract sentences from pairs
        sentences1 = []
        sentences2 = []
        
        for pair in pairs:
            if isinstance(pair, list) and len(pair) == 2:
                sentences1.append(pair[0])
                sentences2.append(pair[1])
            elif isinstance(pair, dict):
                sentences1.append(pair.get('sentence1', ''))
                sentences2.append(pair.get('sentence2', ''))
        
        if not sentences1:
            return jsonify({"error": "Invalid pairs format"}), 400
        
        # Get embeddings for all sentences
        all_sentences = sentences1 + sentences2
        embeddings = model.encode(all_sentences)
        
        # Calculate cosine similarity
        embeddings1 = embeddings[:len(sentences1)]
        embeddings2 = embeddings[len(sentences1):]
        
        similarities = []
        for emb1, emb2 in zip(embeddings1, embeddings2):
            # Cosine similarity
            sim = np.dot(emb1, emb2) / (np.linalg.norm(emb1) * np.linalg.norm(emb2))
            similarities.append(float(sim))
        
        return jsonify({"similarities": similarities})
        
    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    print("Starting SentenceTransformer server...")
    print(f"Model: all-MiniLM-L6-v2")
    print("Available endpoints:")
    print("  GET  /health")
    print("  POST /encode")
    print("  POST /similarity")
    app.run(host='0.0.0.0', port=5000)
