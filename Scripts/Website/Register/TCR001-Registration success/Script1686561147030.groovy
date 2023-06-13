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

WebUI.navigateToUrl('https://demo-app.online/')
WebUI.delay(1)
WebUI.click(findTestObject('Object Repository/Register/Page_Be a Profressional Talent with Coding.ID/button_Buat Akun'))
WebUI.delay(2)
WebUI.setText(findTestObject('Object Repository/Register/Page_Buat akun dan dapatkan akses di Coding.ID/input_name'), name)
WebUI.delay(2)
WebUI.setText(findTestObject('Object Repository/Register/Page_Buat akun dan dapatkan akses di Coding.ID/input_birth_date'), birthdate)
WebUI.delay(2)
WebUI.setText(findTestObject('Object Repository/Register/Page_Buat akun dan dapatkan akses di Coding.ID/input_email'), email)
WebUI.delay(2)
WebUI.setText(findTestObject('Object Repository/Register/Page_Buat akun dan dapatkan akses di Coding.ID/input_whatsapp'), whatsapp)
WebUI.delay(2)
WebUI.setText(findTestObject('Object Repository/Register/Page_Buat akun dan dapatkan akses di Coding.ID/input_password'), password)
WebUI.delay(2)
WebUI.setText(findTestObject('Object Repository/Register/Page_Buat akun dan dapatkan akses di Coding.ID/input_password_confirmation'), confirmationPassword)
WebUI.delay(2)
WebUI.check(findTestObject('Object Repository/Register/Page_Buat akun dan dapatkan akses di Coding.ID/Checkbox_agreement'))
WebUI.delay(2)
WebUI.click(findTestObject('Object Repository/Register/Page_Buat akun dan dapatkan akses di Coding.ID/button_Daftar'))
WebUI.delay(2)

WebUI.closeBrowser()