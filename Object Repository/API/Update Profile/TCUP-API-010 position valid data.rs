<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TCUP-API-010 position valid data</name>
   <tag></tag>
   <elementGuidId>434b9b6b-94a8-4575-8365-ecba92738a57</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;position&quot;,
      &quot;value&quot;: &quot;QA Engineer&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
      <webElementGuid>8d422f68-22c4-4d5b-bb67-1310ede390dd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>68a0971b-473c-4134-a3b6-7263543227f4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiYzZiNWIzN2ZhYjIxYmIwOTUwMTEzNGIwOTNlOGE5YTZlY2UzMjVjMjUyNjYzY2UwY2M0NzEyNGEwMWE2ZWQ5ZDU3NDRmYWJhNDYwMTM4NWIiLCJpYXQiOjE2ODY4MzA5NjMuOTQ3MDEsIm5iZiI6MTY4NjgzMDk2My45NDcwMTMsImV4cCI6MTcxODQ1MzM2My45NDM4NjQsInN1YiI6IjU4MiIsInNjb3BlcyI6W119.lnya9x3OWklr8XPI7Lo-tf2Dqyxg8DpSHn7d2f6Smd5LWjHFBV3uJtvirxONubBLCbD5AS4b9oWBEcGIb5Xi0GmoBDwSAAqjId8VbT95gSZP4iio2DwSrIpZUcF_LE_ebvLXZmQZ1uO0l9_6eljG-YI2viZwMwbgDheX2U-hhLDNj7BdBoBerjjfDiDI7pVWCkPxpTL13RZ0yBHkhuyKTNVz_KCVqXtoNyDnJiYIjnfcerNw5y_UjmxJh83iYwGzw-DchL8zmvQCD3yome6Sns5EN8XKozay68bpPIvsu974zu8jFWb5oD37nicUIihsvgPWMhjGEBafjzrmKEtJ7KurNm_fUmzehfAe3uI-gyIzJOGmF7d7ucQvrD0kFNHeFeEx9nvMf0dSCDj1ro4H0GefUtLtj2d6a-L7izRoyUuIAFN0dSoojEaUDxK1pcxn9g1hsxrXsgqtqQTdMmQr-yXuG5xk53sdUT0ierkReNVuMRLReT7_jYrY-9vlznIzYMtMXA4nYg4MEf3GA6AaI1c2yh0cGUGigCrFZ86sRHSMjOlv8ZTcd7Sw9l_8SNX6z5JEQZGBqXOa8dhwx-addFM62ToP3-ELVe-inY3-Ny6l95SdM0gPvGHGO82WItZ8B9lzRdnMF7GIRSNp7uxG-WbjlaXGCIHLQr1t_pMsLeI</value>
      <webElementGuid>6a0cb507-5f1b-4cb9-92db-2f450a145839</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://demo-app.online/api/updateprofile</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
