// 全局缓存汇率数据
let cachedRates = {};

// 获取汇率并缓存数据
fetch('/api/rates')
    .then(response => response.json())
    .then(data => {
        cachedRates = data.rates; // 缓存汇率数据
        calculateExchange(); // 默认加载时计算一次
    })
    .catch(error => console.error('获取汇率数据时出错:', error));

// 监听用户输入和选择变化
document.getElementById('amount').addEventListener('input', calculateExchange);
document.getElementById('currency').addEventListener('change', calculateExchange);

// 计算汇率转换
function calculateExchange() {
    const amount = parseFloat(document.getElementById('amount').value) || 0;
    const currency = document.getElementById('currency').value;

    const rate = cachedRates[currency];
    if (rate) {
        const result = amount * rate;
        document.getElementById('result').textContent = `转换后的金额：${result.toFixed(2)} ${currency}`;
    } else {
        document.getElementById('result').textContent = '无法获取汇率，请稍后重试。';
    }
}
