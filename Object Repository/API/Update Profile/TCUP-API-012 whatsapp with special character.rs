<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TCUP-API-012 whatsapp with special character</name>
   <tag></tag>
   <elementGuidId>105fd3cb-0a56-4c67-ad63-577234668808</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiZjFhOTYyZGQzNjU2YmNkMDVkYzRhMTFkMjQ1YzllNzc1YmUzZWI1OTBmODA2ZDgyYTZkMWRmYTZhZWMzYzNlYzU1ZTFkMGEzMjcwOWE3MjgiLCJpYXQiOjE2ODY4OTM2MjkuNTEwNjEzLCJuYmYiOjE2ODY4OTM2MjkuNTEwNjE5LCJleHAiOjE3MTg1MTYwMjkuNTA1OTkzLCJzdWIiOiI1ODIiLCJzY29wZXMiOltdfQ.GsdIY6G-iAPGu8XEX000sjL-_RbtctGDtDyQjY81r9L1hFkMY__I5uYmqZOPvnQLYn42SSJ58U-pFiQOZpWH1f4rGuUbKmeWg5KYWmXdPpu67a_SnKeQ0aNxDgN79KXhhWEOYWd6AWubxShZv20K5noKQZ7Hsebob1sguv1Y54R-jiJ_TkpxMnolC8bmccVZwFKgXpG9XstNZFhZGNNDkzwVuN0LLiXW5nhtShUdD_DpHhbGVfEQ_2sEiPm_OLodBzddr04XMIQcGnHBpQu3znEFlWnDfOU3RhUk2gnz8BJm-lkTHRHlPhnL8p9p671SyHqSEnehgi0RTruA_5lIfZWIGTvHsxl-Lkn5C4yIF8LGzHz4cj7rGVKdq0zHMpt_XP_4BSCad5os17jJLyBtXYDxBzNls6AzawMMALyB88ms4wn-XZJr2wO25dReWjOF4q71Zt8BAU5QDp7gb5TO6j-s9q0BBWkaz1aIgoe5XtbVJQS9NlNogmOykuge1kZ04CGu144qpm80YAfzRVmdW7m38VxymCYRqCV5B69QMklGtnF2BfPrFlKz5iqxZ5SYKcBTgQkwGzYw-WV-KRmVB0yQ42uGkpH9Ps0rWol0FyowY4GATqucwlAQPxn8pN1_j-A7Trci8_2Fa7ETfJrPVmAEm8xK6s-8yRJ4VlBUP9o</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;whatsapp&quot;,
      &quot;value&quot;: &quot;+1234567890+&quot;,
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
      <webElementGuid>d44e150b-27c0-4218-83b9-612eb7d5deda</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>multipart/form-data</value>
      <webElementGuid>7e2cde4b-d196-499e-83c3-4ecc9d908708</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiZjFhOTYyZGQzNjU2YmNkMDVkYzRhMTFkMjQ1YzllNzc1YmUzZWI1OTBmODA2ZDgyYTZkMWRmYTZhZWMzYzNlYzU1ZTFkMGEzMjcwOWE3MjgiLCJpYXQiOjE2ODY4OTM2MjkuNTEwNjEzLCJuYmYiOjE2ODY4OTM2MjkuNTEwNjE5LCJleHAiOjE3MTg1MTYwMjkuNTA1OTkzLCJzdWIiOiI1ODIiLCJzY29wZXMiOltdfQ.GsdIY6G-iAPGu8XEX000sjL-_RbtctGDtDyQjY81r9L1hFkMY__I5uYmqZOPvnQLYn42SSJ58U-pFiQOZpWH1f4rGuUbKmeWg5KYWmXdPpu67a_SnKeQ0aNxDgN79KXhhWEOYWd6AWubxShZv20K5noKQZ7Hsebob1sguv1Y54R-jiJ_TkpxMnolC8bmccVZwFKgXpG9XstNZFhZGNNDkzwVuN0LLiXW5nhtShUdD_DpHhbGVfEQ_2sEiPm_OLodBzddr04XMIQcGnHBpQu3znEFlWnDfOU3RhUk2gnz8BJm-lkTHRHlPhnL8p9p671SyHqSEnehgi0RTruA_5lIfZWIGTvHsxl-Lkn5C4yIF8LGzHz4cj7rGVKdq0zHMpt_XP_4BSCad5os17jJLyBtXYDxBzNls6AzawMMALyB88ms4wn-XZJr2wO25dReWjOF4q71Zt8BAU5QDp7gb5TO6j-s9q0BBWkaz1aIgoe5XtbVJQS9NlNogmOykuge1kZ04CGu144qpm80YAfzRVmdW7m38VxymCYRqCV5B69QMklGtnF2BfPrFlKz5iqxZ5SYKcBTgQkwGzYw-WV-KRmVB0yQ42uGkpH9Ps0rWol0FyowY4GATqucwlAQPxn8pN1_j-A7Trci8_2Fa7ETfJrPVmAEm8xK6s-8yRJ4VlBUP9o</value>
      <webElementGuid>cdc0fd9a-6d8f-49f3-a575-552dbdc6c778</webElementGuid>
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
