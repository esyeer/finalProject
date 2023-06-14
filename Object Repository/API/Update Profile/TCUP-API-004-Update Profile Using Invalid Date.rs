<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TCUP-API-004-Update Profile Using Invalid Date</name>
   <tag></tag>
   <elementGuidId>86fe5597-7113-4d78-a706-98786cc81393</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiZDBmMzUxNDhmNjQxM2JmMGY2YzExMDllNjk4ZWU2ZDQ3ODQyNjRjMWZhZTAzODgzMDc3OTg3YjdlZWRmZWY0YzA3NmQ5NDVhZmI1ZDQwNjIiLCJpYXQiOjE2ODY3MzY5ODUuMDAwODc0LCJuYmYiOjE2ODY3MzY5ODUuMDAwODc5LCJleHAiOjE3MTgzNTkzODQuOTk3MjIxLCJzdWIiOiI2NCIsInNjb3BlcyI6W119.aFDb2elnzaTWNt02T7sebSxTyNjiZRjNvocvMZvVcou82emcKKZjV9Zb75hA_gnLP6DGUs09O4ywZCwPR4U3hzVZY29Qu8VUUUS7esuZkSiKe5oeLk2-IBWTi17zPjr3fl0Ti3cU-qZ9z-zsAv5M7cF41q_nFfF87JdnwHkmdK7OJkpF-l_PVZo90ziuiIFWWNS3QNu0X681GVVDe5N8ZpAaTtM8geNdh4dGLvZahm9n8JFMUMJiZfQaAfQnfw0WxIUQxkm_VbWWgeYS0Oa5GPffF8oXNNMM0nC-sIBxWr_fI38PgD2QAQJlJZ3F9Cznkbrx-_InYqHWKIN_mSaSXlHF-kak9Lwnb1pAdJMRfBv4cyTHgO1JS1rB-_7U3U05EuW-sxaGrT8Nrn6-vY2ifvg8Bf_QBXrZVwiw7on4kV7jeBElr7RYg03tscuxIYaBeSj_TLU1vx-yh660Uh9SfzJtzHPOB-FUW1MGS9lfXLAsqxbnm3iYKNQY2lAq0ZPmFIQg0aSTWM-0bFvmjpLvZ1wCLAbpFtbnT8P6mHz3Z0PQl17Ty6UJmvP6MKX119kD11UzocGj42etVPo_917LHQhighvRPxtKrcBdP67yW48xUjLMafHDBRllu_OfwtrgJ0eVgb9pST1suyYzTrVokKjnkkEMCPppCtpbyLT9EnA</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\n\&quot;name\&quot;:\&quot;Syarif123\&quot;,\n\&quot;whatsapp\&quot;:\&quot;082189913645\&quot;,\n\&quot;birth_date\&quot;:\&quot;InVlid\&quot;\n\n}\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiZDBmMzUxNDhmNjQxM2JmMGY2YzExMDllNjk4ZWU2ZDQ3ODQyNjRjMWZhZTAzODgzMDc3OTg3YjdlZWRmZWY0YzA3NmQ5NDVhZmI1ZDQwNjIiLCJpYXQiOjE2ODY3MzY5ODUuMDAwODc0LCJuYmYiOjE2ODY3MzY5ODUuMDAwODc5LCJleHAiOjE3MTgzNTkzODQuOTk3MjIxLCJzdWIiOiI2NCIsInNjb3BlcyI6W119.aFDb2elnzaTWNt02T7sebSxTyNjiZRjNvocvMZvVcou82emcKKZjV9Zb75hA_gnLP6DGUs09O4ywZCwPR4U3hzVZY29Qu8VUUUS7esuZkSiKe5oeLk2-IBWTi17zPjr3fl0Ti3cU-qZ9z-zsAv5M7cF41q_nFfF87JdnwHkmdK7OJkpF-l_PVZo90ziuiIFWWNS3QNu0X681GVVDe5N8ZpAaTtM8geNdh4dGLvZahm9n8JFMUMJiZfQaAfQnfw0WxIUQxkm_VbWWgeYS0Oa5GPffF8oXNNMM0nC-sIBxWr_fI38PgD2QAQJlJZ3F9Cznkbrx-_InYqHWKIN_mSaSXlHF-kak9Lwnb1pAdJMRfBv4cyTHgO1JS1rB-_7U3U05EuW-sxaGrT8Nrn6-vY2ifvg8Bf_QBXrZVwiw7on4kV7jeBElr7RYg03tscuxIYaBeSj_TLU1vx-yh660Uh9SfzJtzHPOB-FUW1MGS9lfXLAsqxbnm3iYKNQY2lAq0ZPmFIQg0aSTWM-0bFvmjpLvZ1wCLAbpFtbnT8P6mHz3Z0PQl17Ty6UJmvP6MKX119kD11UzocGj42etVPo_917LHQhighvRPxtKrcBdP67yW48xUjLMafHDBRllu_OfwtrgJ0eVgb9pST1suyYzTrVokKjnkkEMCPppCtpbyLT9EnA</value>
      <webElementGuid>32bc48a7-602f-408f-adf8-e5b0d33d4e16</webElementGuid>
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
