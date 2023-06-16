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

WebUI.takeScreenshot()

WebUI.click(findTestObject('Website/Buy Event/Page_Be a Profressional Talent with Coding.ID/btn_Masuk'))

WebUI.takeScreenshot()

WebUI.setText(findTestObject('Object Repository/Website/Buy Event/Page_Masuk untuk dapatkan akses di Coding.ID/input_email'), 
    'ragil.irvandi97@gmail.com')

WebUI.takeScreenshot()

WebUI.setEncryptedText(findTestObject('Object Repository/Website/Buy Event/Page_Masuk untuk dapatkan akses di Coding.ID/input_password'), 
    'QgfuYk5Tsdi8mqoM6vPKkQ==')

WebUI.takeScreenshot()

WebUI.click(findTestObject('Website/Buy Event/Page_Masuk untuk dapatkan akses di Coding.ID/btn_Login'), FailureHandling.STOP_ON_FAILURE)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Buy Event/Page_Be a Profressional Talent with Coding.ID/btn_Events'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Buy Event/Page_Online event bersertifikat dari prakti_f42b96/Day3_Open'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Buy Event/Verify Buy Events/btn1_Beli Tiket'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Buy Event/Verify Buy Events/btn_Lihat Pembelian Saya'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Buy Event/Page_Coding.ID - Cart/btn_Remove'))

WebUI.takeScreenshot()

WebUI.verifyElementPresent(findTestObject('Website/Buy Event/Verify Buy Events/Remove Events In Cart Success'), 
    0)

WebUI.takeScreenshot()

WebUI.closeBrowser()

