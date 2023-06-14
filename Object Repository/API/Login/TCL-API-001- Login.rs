<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TCL-API-001- Login</name>
   <tag></tag>
   <elementGuidId>1e357b83-83cb-49b9-a46f-b573bf8ddfce</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiNDczYjU3ZWY0ODNkMzdhMDc5NjliMTcyZTRjZDg0YTg3YmQzM2YwZWUyMDFhOWM4ZGY5OTRhNGI4ZWQ2ODMxNmUxMzU5YzY3Y2Y5ZmQ1NGQiLCJpYXQiOjE2ODY3MzQ4MTYuOTAwMjMzLCJuYmYiOjE2ODY3MzQ4MTYuOTAwMjM2LCJleHAiOjE3MTgzNTcyMTYuODk3MjQ2LCJzdWIiOiI1ODIiLCJzY29wZXMiOltdfQ.VahypqG2wnQT8zIdJgoibW9gXuF_p-oCzlyuoQFYrEoAROXh4IhSzn1LuLJqVn_OBDPcq9eRilPf_iPYy_61estbel-n91U64wh8N1cLK3oAjGaDcYB0raApvq5SlulFcb5LxV53hct-6izJQeFc8WVyo5cK4KZJ1vzTSkQLZViKSnCYuwxiB9BnrbpD05tAFst8rNWHOXhaB46W26woEkrFhxc4Lx-c6iwKRp3HI3yer5aXL6CJ37ZmugYYeZH5MX894IHrbF8RD0egZyyD_86kFuVm0DSzjoAY5Mbl3bMUZxiWrVbNaWpPTSQCYx_G4_RDP5jeYndmSDHfN5EnVhm_Yp-eRO8H5P067XLj7OLe8mplJodHCTgvOdUGNynlB6bWpeiLblzWcpvj2G5IQuQbtoNKbLfpCagqa1TxD319zeiAKIT3ixQByVZSdkZwIspnJrX6Gbg68PmqMzGT1toWAjziU8xEgoHZwQujR5K8LNgDN0l_e68M0PMZAPC6n6nY6lgFlUP9HwoISqplpAL-MPygo-tHMYYBASpOiXhE_04i0k3k1Pu5oKCGPku4mpbC1MsfPQv8G2vExTeDOkrKcEaLwlnbqUb70AqUPCUtNcLhHtEjw5m03JiDk5ZF06BIh4teHKAF3oRXZRa_dlPVPr1b6ibgdcF4jR89ZN8</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;email\&quot;:\&quot;${email}\&quot;,\n\&quot;password\&quot;:\&quot;${password}\&quot;\n}&quot;,
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
      <webElementGuid>5b423141-da48-42f5-bf4f-47dd7a0989c7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://demo-app.online/api/login</restUrl>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')


String jsonPass =
&quot;&quot;&quot;
{
  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,
  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
  &quot;title&quot;: &quot;Person&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;firstName&quot;: {
      &quot;type&quot;: &quot;string&quot;,
      &quot;description&quot;: &quot;The person's first name.&quot;
    },
    &quot;lastName&quot;: {
      &quot;type&quot;: &quot;string&quot;,
      &quot;description&quot;: &quot;The person's last name.&quot;
    },
    &quot;age&quot;: {
      &quot;description&quot;: &quot;Age in years which must be equal to or greater than zero.&quot;,
      &quot;type&quot;: &quot;integer&quot;,
      &quot;minimum&quot;: 0
    }
  }
}
&quot;&quot;&quot;
boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
