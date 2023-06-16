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

WebUI.click(findTestObject('Object Repository/Website/Update Profile/Page_Be a Profressional Talent with Coding.ID/btn_Masuk'))

WebUI.takeScreenshot()

WebUI.setText(findTestObject('Object Repository/Website/Update Profile/Page_Masuk untuk dapatkan akses di Coding.ID/input_Email'), 
    'ragil.irvandi97@gmail.com')

WebUI.takeScreenshot()

WebUI.setEncryptedText(findTestObject('Object Repository/Website/Update Profile/Page_Masuk untuk dapatkan akses di Coding.ID/input_Password'), 
    'QgfuYk5Tsdi8mqoM6vPKkQ==')

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Update Profile/Page_Masuk untuk dapatkan akses di Coding.ID/button_Login'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Update Profile/Page_Be a Profressional Talent with Coding.ID/Logo_User'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Update Profile/Page_Be a Profressional Talent with Coding.ID/btn_My Account'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Update Profile/Page_Coding.ID - Dashboard/btn_Profil'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Update Profile/Page_Coding.ID - Dashboard/btn_Edit Profile'))

WebUI.takeScreenshot()

WebUI.setText(findTestObject('Website/Update Profile/Page_Coding.ID - Dashboard/input_WhatsApp'), '')

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Website/Update Profile/Page_Coding.ID - Dashboard/btn_Save Changes'))

WebUI.takeScreenshot()

WebUI.verifyTextPresent('The whatsapp field is required', false)

WebUI.takeScreenshot()

WebUI.closeBrowser()

