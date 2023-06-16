<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TCUP-API-001-Update Profile</name>
   <tag></tag>
   <elementGuidId>fc608935-85ab-4802-9439-8cf4179ca100</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiY2NmM2I0MDcwODcwMTUxYzgxYWE3ZmNmZmY1NmZiYTY5YjU0ZGQ4ODgxYjQ0YTI2ODU0M2RjYzIzMGQ4YjAyMDE2YmM1MTM0YTY3OGMzZDciLCJpYXQiOjE2ODY3NzA3MjMuODQwNDczLCJuYmYiOjE2ODY3NzA3MjMuODQwNDc1LCJleHAiOjE3MTgzOTMxMjMuODM3NTA2LCJzdWIiOiI1ODIiLCJzY29wZXMiOltdfQ.CrV0HtPIrs5jqwQso2WlJGNOpwFQYgAIFMed6MhB6rjZMrj3wI5OvTRqStbmc5L1aICiCdW8hWLjTB-ZZ9-m074491b5y-pIzxmC-UcQN6KInSu3ZXulE3l980t2Mkq-9857BqhbFbeRmVjfwghjW_0D29Ux_NyTxNtK5DjwNUCCdaNkUG9GnSVfJ-B0BzltOOuheKKyXU1ssIgeE2AdSLrS4jjk2tDzUyHhaNdR7YmvsSEfTk4Ct6L5BWVISx5pnFFFKzE4QUOwjpcibOxqdKDN9mtMCp--NQmT5_M98E0y38H9CTEHaHwSX5xxV51TzE8Yhxw62gqvUWrRBM8b1R8otywfxeMWdSLORhseB-4rV9FuS-t47sklZ4GKISff0qKEXDXXE1pz0P7JJSuI6v468MwfCX67HOU7x2LOq8EWFqp1ktI_G7Zc5gu-rPlBCuz1iqBWDiwohOM5vMmQPEcuE0nLHxiYxcmmMGpRWsRKSJOr2nyVx3tlMZsIIelJr_B1pUz-by7dniDg4LDx0cK7zMKt2RuiUGUhJiako9dStDD0YDvLLIkHbazORaIdNc0yBbGZ9fo9sPvyxrzN8Cfdtd3P6MfL7IYPzvkU58IKyzRuKJFHglESsrysZ8YkN2y89DX5AffdkBb-iOpbM0mtkQG_uT5mr-Dhpr9S2aw</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;name\&quot;:\&quot;${name}\&quot;,\n\&quot;whatsapp\&quot;:\&quot;${whatsapp}\&quot;,\n\&quot;birth_date\&quot;:\&quot;${birthdate}\&quot;,\n\&quot;photo\&quot;:\&quot;\&quot;,\n\&quot;bio\&quot;:\&quot;${bio}\&quot;,\n\&quot;position\&quot;:\&quot;${position}\&quot;\n  \n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b375a42a-4534-4755-af69-6aa47f941402</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiY2NmM2I0MDcwODcwMTUxYzgxYWE3ZmNmZmY1NmZiYTY5YjU0ZGQ4ODgxYjQ0YTI2ODU0M2RjYzIzMGQ4YjAyMDE2YmM1MTM0YTY3OGMzZDciLCJpYXQiOjE2ODY3NzA3MjMuODQwNDczLCJuYmYiOjE2ODY3NzA3MjMuODQwNDc1LCJleHAiOjE3MTgzOTMxMjMuODM3NTA2LCJzdWIiOiI1ODIiLCJzY29wZXMiOltdfQ.CrV0HtPIrs5jqwQso2WlJGNOpwFQYgAIFMed6MhB6rjZMrj3wI5OvTRqStbmc5L1aICiCdW8hWLjTB-ZZ9-m074491b5y-pIzxmC-UcQN6KInSu3ZXulE3l980t2Mkq-9857BqhbFbeRmVjfwghjW_0D29Ux_NyTxNtK5DjwNUCCdaNkUG9GnSVfJ-B0BzltOOuheKKyXU1ssIgeE2AdSLrS4jjk2tDzUyHhaNdR7YmvsSEfTk4Ct6L5BWVISx5pnFFFKzE4QUOwjpcibOxqdKDN9mtMCp--NQmT5_M98E0y38H9CTEHaHwSX5xxV51TzE8Yhxw62gqvUWrRBM8b1R8otywfxeMWdSLORhseB-4rV9FuS-t47sklZ4GKISff0qKEXDXXE1pz0P7JJSuI6v468MwfCX67HOU7x2LOq8EWFqp1ktI_G7Zc5gu-rPlBCuz1iqBWDiwohOM5vMmQPEcuE0nLHxiYxcmmMGpRWsRKSJOr2nyVx3tlMZsIIelJr_B1pUz-by7dniDg4LDx0cK7zMKt2RuiUGUhJiako9dStDD0YDvLLIkHbazORaIdNc0yBbGZ9fo9sPvyxrzN8Cfdtd3P6MfL7IYPzvkU58IKyzRuKJFHglESsrysZ8YkN2y89DX5AffdkBb-iOpbM0mtkQG_uT5mr-Dhpr9S2aw</value>
      <webElementGuid>ba593d41-ee90-4ce9-9a15-6f8145bf01dc</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://demo-app.online/api/updateprofile</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
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
