let ratesCache = null;

// 页面加载时获取汇率并缓存
fetch('/api/rates')
    .then(response => response.json())
    .then(data => {
        ratesCache = data;
        calculateExchange();
    });

// 监听用户输入和选择
document.getElementById('amount').addEventListener('input', calculateExchange);
document.getElementById('currency').addEventListener('change', calculateExchange);

function calculateExchange() {
    const amount = parseFloat(document.getElementById('amount').value) || 0;
    const currency = document.getElementById('currency').value;

    if (ratesCache && ratesCache[currency]) {
        const rate = ratesCache[currency];
        const result = amount * rate;
        document.getElementById('result').textContent = `转换后的金额：${result.toFixed(2)} ${currency}`;
    }
}