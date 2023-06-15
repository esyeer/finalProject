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

Mobile.startApplication('Data Files\\DemoAppV2.apk', true)

Mobile.delay(10)

Mobile.tap(findTestObject('Object Repository/Mobile/Login/btn_loginHere'), 0)

Mobile.delay(2)

Mobile.setText(findTestObject('Object Repository/Mobile/Login/field_mail'), 'faceluck04256@gmail.com', 0)

Mobile.setText(findTestObject('Object Repository/Mobile/Login/field_pass'), '@Qwerty123', 0)

Mobile.delay(2)

Mobile.tap(findTestObject('Object Repository/Mobile/Login/Button_Login'), 0)

Mobile.delay(5)

Mobile.verifyElementNotExist(findTestObject('Object Repository/Mobile/Login/btn_loginHere'), 5)

Mobile.delay(3)

Mobile.tap(findTestObject('Object Repository/Mobile/Update Profile/Button_Profile'), 0)

Mobile.verifyElementText(findTestObject('Mobile/Update Profile/ValidateSuccessProfile'), ' Profile ')

Mobile.delay(3)

Mobile.verifyElementText(findTestObject('Mobile/Update Profile/ValidateSuccessInformation'), 'Detail Information')

Mobile.delay(2)

Mobile.tap(findTestObject('Object Repository/Mobile/Update Profile/Gear_Icon'), 0)

Mobile.delay(2)

Mobile.tap(findTestObject('Object Repository/Mobile/Update Profile/Edit_Profile'), 0)

Mobile.delay(2)

//Mobile.setText(findTestObject('Object Repository/Mobile/Update Profile/Field_Name'), "syarif hidayatullah", 0)

//Mobile.delay(2)

Mobile.setText(findTestObject('Object Repository/Mobile/Update Profile/Field_Phone'), "082189913", 0)

//Mobile.delay(2)

//Mobile.setText(findTestObject('Object Repository/Mobile/Update Profile/Field_Birthdate'), "29-Nov-1998", 0)

Mobile.verifyElementAttributeValue(findTestObject('Object Repository/Mobile/Update Profile/Button_Save Changes'), 'Clickable', true, 0)

Mobile.delay(2)

Mobile.tap(findTestObject('Object Repository/Mobile/Update Profile/Button_Save Changes'), 0)

Mobile.delay(2)

Mobile.verifyElementExist(findTestObject('Object Repository/Mobile/Update Profile/PopUP_SuccessChanges'), 0)

Mobile.verifyElementText(findTestObject('Object Repository/Mobile/Update Profile/Validate_SuccessChanges'), 'Your profile has been successfully updated.')

Mobile.tap(findTestObject('Object Repository/Mobile/Update Profile/btn_popUp_Okay'), 0)

Mobile.verifyElementText(findTestObject('Mobile/Update Profile/ValidateSuccessProfile'), ' Profile ')

//Mobile.delay(3)

Mobile.verifyElementText(findTestObject('Mobile/Update Profile/ValidateSuccessInformation'), 'Detail Information')

Mobile.delay(2)

//Mobile.delay(1)

Mobile.closeApplication()

