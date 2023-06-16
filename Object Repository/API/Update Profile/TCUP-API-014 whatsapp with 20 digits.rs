<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TCUP-API-014 whatsapp with 20 digits</name>
   <tag></tag>
   <elementGuidId>b5644db6-63db-4cc9-a44d-772881d8e05e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiOWQ5YTAxZTBlM2JiNDRlODdiMzdkMzk4MjMyYjY0NGE1YWNhNzIwZDAyYjFjOTIxYWM2NWI3ZmU2NmE3MTA3MGU2OTc5Mjc2MjFiZmYzOTQiLCJpYXQiOjE2ODY4OTM3NDIuNTU0NTA1LCJuYmYiOjE2ODY4OTM3NDIuNTU0NTExLCJleHAiOjE3MTg1MTYxNDIuNTQ5NzgxLCJzdWIiOiI1ODIiLCJzY29wZXMiOltdfQ.E-UpfR6nCiqKLc5ESxsaj0s_SMEEhX6Onr-mtYiU5m1To5vcAnPWqfp6GotdvwI3aOVb_yRXqprw33cq6mfu0IspA6kHi33ZcVziPO65o537hctMYpH-_6l3NRU4yDongdlkRMeCaoGdPt01s4xO2ea-gwb0TrLrnQDEcmR_6PyhBH-4H_5lz9yoGtZc2kLUu67iLj93C-JRavYpbXbm2gYitHtsbcNQG7CK68EyISdbRRW_3qbV8WvHsmRxa8dLsnCQzxrm9xJi6HIuy5JsWYF7Q9AAXS6fPXIwg0fsyKtvcPCAkxlS4n9WvpJBRhQJO9_mseLlatEWcqEFrik_kDSNRsTtRIVi0bAyCz5kWdzkVmWIKtEx1AIVur91nBXyUA_wV64Uu5VS68EgKQ8Q0A2fb3fXJ2TFTVWMLgLI7Ql0vcqszGz4rch6GRHVo7PZ2yPqw1MQAAlu9xGwOw4Gn98vhOrx693sTJGG3dfWrBR6xamoDwK6Ladvxx26Ro6UEPIzd70pqRR4B-2PEhnHrkDh01Oe0aKCtIQjr0xiiW1FLlwIWCjGEVjAjQhllV_u2W3BBymWI92jI7O8nAlndKmT6ucApZ9YwZZyiHM4UIfM3r98DYUM1jnGtPahZjvWRWkaGnIBqAjEUBFIh7ZAau-N4R1WxhQbr01wq44g2T4</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;whatsapp&quot;,
      &quot;value&quot;: &quot;12345678901234567890&quot;,
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
      <webElementGuid>a2b7899a-a334-405a-8ad0-0e79c6b2dd08</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>81607137-7f56-400b-94fb-e569cd1a5db7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiOWQ5YTAxZTBlM2JiNDRlODdiMzdkMzk4MjMyYjY0NGE1YWNhNzIwZDAyYjFjOTIxYWM2NWI3ZmU2NmE3MTA3MGU2OTc5Mjc2MjFiZmYzOTQiLCJpYXQiOjE2ODY4OTM3NDIuNTU0NTA1LCJuYmYiOjE2ODY4OTM3NDIuNTU0NTExLCJleHAiOjE3MTg1MTYxNDIuNTQ5NzgxLCJzdWIiOiI1ODIiLCJzY29wZXMiOltdfQ.E-UpfR6nCiqKLc5ESxsaj0s_SMEEhX6Onr-mtYiU5m1To5vcAnPWqfp6GotdvwI3aOVb_yRXqprw33cq6mfu0IspA6kHi33ZcVziPO65o537hctMYpH-_6l3NRU4yDongdlkRMeCaoGdPt01s4xO2ea-gwb0TrLrnQDEcmR_6PyhBH-4H_5lz9yoGtZc2kLUu67iLj93C-JRavYpbXbm2gYitHtsbcNQG7CK68EyISdbRRW_3qbV8WvHsmRxa8dLsnCQzxrm9xJi6HIuy5JsWYF7Q9AAXS6fPXIwg0fsyKtvcPCAkxlS4n9WvpJBRhQJO9_mseLlatEWcqEFrik_kDSNRsTtRIVi0bAyCz5kWdzkVmWIKtEx1AIVur91nBXyUA_wV64Uu5VS68EgKQ8Q0A2fb3fXJ2TFTVWMLgLI7Ql0vcqszGz4rch6GRHVo7PZ2yPqw1MQAAlu9xGwOw4Gn98vhOrx693sTJGG3dfWrBR6xamoDwK6Ladvxx26Ro6UEPIzd70pqRR4B-2PEhnHrkDh01Oe0aKCtIQjr0xiiW1FLlwIWCjGEVjAjQhllV_u2W3BBymWI92jI7O8nAlndKmT6ucApZ9YwZZyiHM4UIfM3r98DYUM1jnGtPahZjvWRWkaGnIBqAjEUBFIh7ZAau-N4R1WxhQbr01wq44g2T4</value>
      <webElementGuid>0fec97c7-0589-4df4-ba63-0f4f757555a5</webElementGuid>
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
