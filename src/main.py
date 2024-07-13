import requests

headers = {
    "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15",
    "Cookie": "gzid=dst_XXXX1_07_13_dst_XXXX1; nowurl=https%3A//www.x027.com/e_ldstb7_2.html%3Fgzid%3Ddst_XXXX1_07_13_dst_XXXX1%26t%3DjvZWw; ASPSESSIONIDCQDQSRAT=FAKJCMJANEKFEOEDJPDKHIOG; ASPSESSIONIDSSDRACRQ=GLIBANMAIIHPFFOPDCAOGKAN",
}

datas = {
    "business_id": 31446,
    "pro_id": 47044,
    "cpa": 135,
    "orderid": "no.7172085547479854556",
    "num": 1,
    "product": "%D2%BB%B4%CE%BC%AF%C6%EB%A1%B6%B5%DA%CB%C4%CC%D7%C8%CB%C3%F1%B1%D2%A1%B7%A3%AC%C6%B7%CF%E0%C9%CF%B3%CB%A3%AC%CE%B2%CB%C4%CF%E0%CD%AC%A3%AC%D4%AD%B0%E6%D4%AD%B3%AE%A3%AC%B2%BB%BF%C9%D4%D9%C9%FA%A3%A11%D5%DB%C7%C0%B9%BA%D6%D0%A3%AC%C8%AB%CC%D710%C3%B6%A3%AC%D4%D9%D4%F9%CB%CD%D3%B2%B1%D250%C3%B6%A3%AC%B9%B260%C3%B61990%D4%AA",
    "zfbprice": 199,
    "price": 199,
    "yh_price": 1791,
    "name": "%D5%C5%C8%FD",
    "mob": 13542422341,
    "province": "%B1%B1%BE%A9",
    "city": "%B1%B1%BE%A9%CA%D0",
    "area": "%D1%D3%C7%EC%CF%D8",
    "proh": "%B1%B1%BE%A9",
    "cityh": "%B1%B1%BE%A9%CA%D0",
    "areah": "%D1%D3%C7%EC%CF%D8",
    "address": "ABCD",
    "pay": "cod",
    "ty": 1,
}

res = requests.post("https://www.x027.com/notorder_vip.asp", headers=headers, data=datas)
print(res.text)