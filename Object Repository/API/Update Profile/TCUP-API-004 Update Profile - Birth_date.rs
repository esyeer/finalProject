<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TCUP-API-004 Update Profile - Birth_date</name>
   <tag></tag>
   <elementGuidId>06f9b7da-b7b4-4a41-bca8-911f34e420d7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiZTM3NThjZDI0YzY3ZDMxMTNmOWE2YTgyMDRiY2ZjNzhjNzUwNjgyMTllZTM4NzNmOGEyMDJiZTgyNjhhODJmYzk0NzhmYTI2MDAwZTU1ZDQiLCJpYXQiOjE2ODY4OTQ4MDQuNDY5MTg5LCJuYmYiOjE2ODY4OTQ4MDQuNDY5MTkyLCJleHAiOjE3MTg1MTcyMDQuNDY2MDIzLCJzdWIiOiI1ODIiLCJzY29wZXMiOltdfQ.crUKB7_cl2RS_HYGc78F7xO6CaVWXsOO1XukAC-wxr31qmMQr7Dwh60i9bkGpVafnoVchrWw7s0S4fOcS1D9htXD-XFUzeMSuq-bhIV1__vxYrEMMFdcDl3HdMld-jSCPlrMnrm3-0cPzv5TPsJ1dc5vi_CQn7Y90S0MqsZYcez6y6dKs4OXkBsSKMMdKaTU8EwqN-AGvokeUat46c1NhRfrIhPk02MhWREpWyim7HfCMx6FRspC9m2wJLZV0TIiwUYrgSITm3mMfnB_fb7suNoRXmaVX2zT70Glnm4BUEieQH5Z8EQhzL0G1gXBS9iMHj5Pua5mZsiZJijArbf0dngPHbBeKlvZKWukMCO85_BxpjK6XrU5R2_x3qo0MX4u-jUBICxQHAnbxK1StKhZmJ94fxxgzgAzBaaYpn4jMC5EeZVIOBWwxkol71V2k12X2HdTBC56d8dUzLBa8P25ctTEPZP_Mt56GjVvL1BeyN4zavmgEBvR8KHDBgQiKpfic0DoaIZ2lgawmd40llQ1GGw5pKPRnKjiYUjh1U1O-yJg-ASX8KnmOjk4jIK8zBlNoMAOcHZjVoTU765pOKsxqNNPG03jAqS0kRx0Odp5B7yJlHnRIlZZGoRIFfFZtpArxfjqLNC6G6-wSVU4O8ikx-l4l2e2B3YcLjCP1o_8sNw</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;birth_date&quot;,
      &quot;value&quot;: &quot;2010-09-02&quot;,
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
      <webElementGuid>d39d2942-72d7-42d5-a79a-925052065a6c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiZTM3NThjZDI0YzY3ZDMxMTNmOWE2YTgyMDRiY2ZjNzhjNzUwNjgyMTllZTM4NzNmOGEyMDJiZTgyNjhhODJmYzk0NzhmYTI2MDAwZTU1ZDQiLCJpYXQiOjE2ODY4OTQ4MDQuNDY5MTg5LCJuYmYiOjE2ODY4OTQ4MDQuNDY5MTkyLCJleHAiOjE3MTg1MTcyMDQuNDY2MDIzLCJzdWIiOiI1ODIiLCJzY29wZXMiOltdfQ.crUKB7_cl2RS_HYGc78F7xO6CaVWXsOO1XukAC-wxr31qmMQr7Dwh60i9bkGpVafnoVchrWw7s0S4fOcS1D9htXD-XFUzeMSuq-bhIV1__vxYrEMMFdcDl3HdMld-jSCPlrMnrm3-0cPzv5TPsJ1dc5vi_CQn7Y90S0MqsZYcez6y6dKs4OXkBsSKMMdKaTU8EwqN-AGvokeUat46c1NhRfrIhPk02MhWREpWyim7HfCMx6FRspC9m2wJLZV0TIiwUYrgSITm3mMfnB_fb7suNoRXmaVX2zT70Glnm4BUEieQH5Z8EQhzL0G1gXBS9iMHj5Pua5mZsiZJijArbf0dngPHbBeKlvZKWukMCO85_BxpjK6XrU5R2_x3qo0MX4u-jUBICxQHAnbxK1StKhZmJ94fxxgzgAzBaaYpn4jMC5EeZVIOBWwxkol71V2k12X2HdTBC56d8dUzLBa8P25ctTEPZP_Mt56GjVvL1BeyN4zavmgEBvR8KHDBgQiKpfic0DoaIZ2lgawmd40llQ1GGw5pKPRnKjiYUjh1U1O-yJg-ASX8KnmOjk4jIK8zBlNoMAOcHZjVoTU765pOKsxqNNPG03jAqS0kRx0Odp5B7yJlHnRIlZZGoRIFfFZtpArxfjqLNC6G6-wSVU4O8ikx-l4l2e2B3YcLjCP1o_8sNw</value>
      <webElementGuid>5e8b1fa5-58ee-41c1-839f-1c74b7d5b3d7</webElementGuid>
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
