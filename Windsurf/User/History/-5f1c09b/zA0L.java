package com.jvmd.walto_backend.model;

public enum EDonateType {
    VIP_STATUS("VIP статус", 100),
    PREMIUM_STATUS("Premium статус", 250),
    CREATOR_PACK("Пакет создателя", 500),
    DIAMOND_SUPPORTER("Diamond поддерживающий", 1000),
    EMERALD_SUPPORTER("Emerald поддерживающий", 2500),
    CUSTOM_AMOUNT("Произвольная сумма", 0);

    private final String displayName;
    private final Integer defaultPrice;

    EDonateType(String displayName, Integer defaultPrice) {
        this.displayName = displayName;
        this.defaultPrice = defaultPrice;
    }

    public String getDisplayName() {
        return displayName;
    }

    public Integer getDefaultPrice() {
        return defaultPrice;
    }
}

