use anyhow::Result;
use scraper::{Html, Selector};

pub fn parse_saldo(resp: &str) -> Result<Vec<String>> {
    let doc = Html::parse_document(resp);
    let table_selector = Selector::parse("tr[bgcolor='#FFFFFF']>td").unwrap();

    let mut text : Vec<String> = vec![];

    for td in doc.select(&table_selector) {
        let t = td.text().collect::<String>();
        text.push(t);
    }
    Ok(text)
}

mod saldo_test {
    use anyhow::Result;
    use crate::saldo_parser::parse_saldo;
#[test]
    fn parse_saldo_test() -> Result<()>{
        let html_text = r#"
            <!DOCTYPE html PUBLIC "-//WAPFORUM//DTD XHTML Mobile 1.0//EN" "http://www.wapforum.org/DTD/xhtml-mobile10.dtd">
            <html xmlns="http://www.w3.org/1999/xhtml">
            <head>
            <title>KlikBCA</title>
            <style type="text/css">
            <!--
            hr { margin-top: auto; margin-bottom: auto;}
        .BlueRule { border: 1px solid #4f53b6;}
        .OranRule { border: 1px solid #e7d300;}
        -->
            </style>
            <link href="style.css" rel="stylesheet" type="text/css" />
            <meta name="viewport" content="width=device-width; initial-scale=1.0; maximum-scale=1.0;"/>
            <meta http-equiv="Content-Type" content="application/xhtml+xml; charset=UTF-8" />
            <meta http-equiv="cache-control" content="no-cache">
            <meta http-equiv="expires" content="-1" />
            </head>
            <body onload="fncTimer();">

            <div id="wrapper">
            <div id="header">
            <div id="logo" style="background:#FFFFFF url(images/bg_top_2x55.gif) repeat-x top; height:55px; color:#FFFFFF">
            <img src="images/logo_bca_64x55.gif" alt="BCA-lite"/>
            </div>
            <div style="position: absolute; top: 30px; left: 65px; background-color: #4f53b6;">
            <font color="white">Tanggal&nbsp;:&nbsp;29-05-2020</font></div>
            <div style="background:#e7d300; height:3px;"></div>
            </div>
            <table width="100%" cellpadding="0" cellspacing="0" border="0">
            <tr><td>
            <div id="pagebody" align="left">


            <span class="blue">
            <table cellpadding="0" cellspacing="0" border="0" width="100%" class="cheader">
            <tr bgcolor="\#4f54b2"><td colspan="2" height="5px;"></td></tr>
            <tr bgcolor="\#4f54b2">
            <td width="10"></td>
            <td style="color:#FFFFFF" valign="middle"><font size="3"><strong>INFORMASI REKENING - INFORMASI SALDO</strong></td>
            </tr>
            <tr bgcolor="\#4f54b2"><td colspan="2" height="5px;"></td></tr>
            </table>
            <table cellpadding="3" cellspacing="0" border="0" width="100%">
            <tr>
            <td width="5">&nbsp;</td>
            <td>
            <table width="100%" cellpadding="5" cellspacing="5">
            <tr>
            <td bgcolor="\#F0F0F0" width="60%"><font color="\#0000a7"><b>REKENING</td>
            <td colspan="2" bgcolor="\#F0F0F0"><font color="\#0000a7"><b>SALDO EFEKTIF</td>
            </tr>
            <tr bgcolor='#FFFFFF'>
            <td><font size='1' color='#0000a7'><b>888072345</td>
            <td width='5%'><font size='1' color='#0000a7'><b>IDR</td>
            <td align='right'><font size='1' color='#0000a7'><b>659,300,943.52</td>
            </tr>

            <!--<tr>
            <td height="21" valign="top"> TAHAPAN<br />5270813205</td>
            <td valign="top">Rp. 99.000.000.000,00</td>
            </tr>-->
            </table>
            </td>
            <td width="5">&nbsp;</td>
            </tr>
            </table>
            </span>
            <form name="iBankForm" action="\#" method="post"></form>

            </div>
            </td></tr>
            </table>
            <div id="footer" valign="middle">
            <table border="0" cellpadding="0" cellspacing="0" width="100%">
            <tr>
            <td width="10" height="35">&nbsp;</td>
            <td valign="middle" align="left" width="100"><a href="authentication.do?value(actions)=menu" style="text-decoration:none; color:#FFFFFF;" onmouseout="this.style.textDecoration='none'" onmouseover="this.style.textDecoration='underline'">MENU UTAMA</a></td>
            <td valign="middle" align="left" width="50"><a href="authentication.do?value(actions)=logout" style="text-decoration:none; color:#FFFFFF;" onmouseout="this.style.textDecoration='none'" onmouseover="this.style.textDecoration='underline'">LOGOUT</a></td>
            <td>&nbsp;</td>
            </tr>
            </table>
            </div>
            </div>

            </body>
            </html>

            <script language="javascript" type="text/javascript">
            var ms = 0;
        var then = new Date();
        function fncTimer() {
            setTimeout("fncTimer();", 1000);
            ynow = new Date();
            ms = ynow.getTime() - then.getTime();
            if (ms>=300000) {
                document.iBankForm.action = "authentication.do?value(actions)=logout";
                document.iBankForm.submit();
                window.location="https://m.klikbca.com/";
            }
            window.status=parseInt(ms/1000) + " seconds idle";
        }
        </script>
            "#;

        let text = parse_saldo(html_text)?;
        Ok(assert_eq!(text, vec!["888072345", "IDR", "659,300,943.52"]))
    }
}