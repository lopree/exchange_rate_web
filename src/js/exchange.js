let exchangeRates = {};

// 在页面加载时获取汇率数据并缓存
window.addEventListener('DOMContentLoaded', async () => {
    try {
        const response = await fetch('/api/rates');
        exchangeRates = await response.json();
        console.log("获取到的汇率数据:", exchangeRates);
    } catch (error) {
        console.error("获取汇率数据失败:", error);
    }
});

// 计算汇率
function calculateExchange() {
    const amount = parseFloat(document.getElementById('amount').value) || 0;
    const currency = document.getElementById('currency').value;
    const rate = exchangeRates[currency];

    if (rate) {
        const result = amount * rate;
        document.getElementById('result').textContent = `转换后的金额：${result.toFixed(2)} ${currency}`;
    } else {
        document.getElementById('result').textContent = "无法获取该货币的汇率";
    }
}

// 监听输入框和下拉菜单的变化
document.getElementById('amount').addEventListener('input', calculateExchange);
document.getElementById('currency').addEventListener('change', calculateExchange);