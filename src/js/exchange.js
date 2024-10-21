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
