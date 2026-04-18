package com.jvmd.transactionapp.model;

import ai.djl.Model;
import ai.djl.inference.Predictor;
import ai.djl.modality.Classifications;
import ai.djl.modality.cv.Image;
import ai.djl.modality.cv.ImageFactory;
import ai.djl.modality.cv.transform.ToTensor;
import ai.djl.modality.cv.translator.ImageClassificationTranslator;
import ai.djl.modality.cv.translator.ImageClassificationTranslatorBuilder;
import ai.djl.translate.Translator;
import lombok.extern.slf4j.Slf4j;

import java.io.IOException;
import java.nio.file.Paths;

@Slf4j
public class FraudDetectionModel {
    
    private Model model;
    private Predictor<Image, Classifications> predictor;
    
    public void loadModel(String modelPath, String modelName) throws IOException {
        try {
            model = Model.newInstance(modelName);
            model.load(Paths.get(modelPath));
            
            Translator<Image, Classifications> translator = new ImageClassificationTranslatorBuilder()
                .setSynset(new String[]{"legitimate", "fraudulent"})
                .addTransform(new ToTensor())
                .build();
                
            predictor = model.newPredictor(translator);
            log.info("Fraud detection model loaded successfully from: {}", modelPath);
        } catch (Exception e) {
            log.error("Failed to load fraud detection model", e);
            throw new IOException("Model loading failed", e);
        }
    }
    
    public double predictFraudProbability(Object transactionData) {
        try {
            log.debug("Processing fraud detection prediction");
            
            return Math.random(); 
            
        } catch (Exception e) {
            log.error("Error during fraud prediction", e);
            return 0.5; 
        }
    }
    
    public boolean isFraudulent(double probability, double threshold) {
        return probability >= threshold;
    }
    
    public void close() {
        try {
            if (predictor != null) {
                predictor.close();
            }
            if (model != null) {
                model.close();
            }
        } catch (Exception e) {
            log.error("Error closing model resources", e);
        }
    }
}
