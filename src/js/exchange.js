// 获取汇率并生成选项
function loadExchangeRates() {
    const loadingIndicator = document.getElementById('loading');
    const errorMessage = document.getElementById('error');
    loadingIndicator.style.display = 'block'; // Show loading indicator
    errorMessage.style.display = 'none'; // Hide error message

    fetch('/api/rates')
        .then(response => {
            if (!response.ok) {
                throw new Error('网络错误'); // Throw an error if the response is not ok
            }
            return response.json();
        })
        .then(data => {
            const select = document.getElementById('currency');
            const rates = data.rates;
            const currencyInfo = {
                "USD": "美元",
                "EUR": "欧元",
                "GBP": "英镑",
                "JPY": "日元",
                "AUD": "澳大利亚元",
                "CAD": "加拿大元",
                "CNY": "人民币",
                // 添加更多汇率和中文名
            };

            // 生成下拉菜单选项
            for (const [code, rate] of Object.entries(rates)) {
                const option = document.createElement('option');
                option.value = code;
                option.text = `${code} - ${currencyInfo[code] || '未知'} (${rate})`;
                select.appendChild(option);
            }

            // 默认选中美元
            select.value = 'USD';
        })
        .catch(error => {
            errorMessage.textContent = `错误: ${error.message}`; // Display error message
            errorMessage.style.display = 'block'; // Show error message
        })
        .finally(() => {
            loadingIndicator.style.display = 'none'; // Hide loading indicator
        });
}

// Call the function to load exchange rates
loadExchangeRates();

// 监听用户输入和选择
document.getElementById('amount').addEventListener('input', calculateExchange);
document.getElementById('currency').addEventListener('change', calculateExchange);

function calculateExchange() {
    const amount = parseFloat(document.getElementById('amount').value) || 0;
    const currency = document.getElementById('currency').value;

    fetch('/api/rates')
        .then(response => response.json())
        .then(data => {
            const rate = data.rates[currency];
            const result = amount * rate;
            document.getElementById('result').textContent = `转换后的金额：${result.toFixed(2)} ${currency}`;
        });
}