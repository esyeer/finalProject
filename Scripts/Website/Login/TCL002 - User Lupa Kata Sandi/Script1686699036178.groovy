import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://demo-app.coding.id/')

WebUI.click(findTestObject('Object Repository/Login/Page_Be a Profressional Talent with Coding.ID/btn_Masuk'))

WebUI.click(findTestObject('Object Repository/Login/Page_Masuk untuk dapatkan akses di Coding.ID/btn_Lupa kata sandi'))

WebUI.setText(findTestObject('Object Repository/Login/Page_Lupa password akun Coding.ID kamu Rese_b3cffc/txt_Lupa Kata Sandi_email'), 
    'ragil.irvandi97@gmail.com')

WebUI.click(findTestObject('Object Repository/Login/Page_Lupa password akun Coding.ID kamu Rese_b3cffc/btn_Kirim link'))

WebUI.verifyElementPresent(findTestObject('Login/Verify Login/Page_Lupa password akun Coding.ID kamu Reset disini/p_Kami telah mengirimkan link untuk mengatur ulang kata sandi anda.Silahkan periksa email anda'), 
    1)

