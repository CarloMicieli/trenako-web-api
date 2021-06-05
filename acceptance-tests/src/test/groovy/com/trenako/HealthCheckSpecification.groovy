package com.trenako

import io.micronaut.http.HttpStatus
import io.micronaut.http.client.RxHttpClient
import io.micronaut.http.client.annotation.Client
import io.micronaut.test.extensions.spock.annotation.MicronautTest
import spock.lang.Specification

import javax.inject.Inject

@MicronautTest
class HealthCheckSpecification extends Specification {

    @Client("http://localhost:5000")
    @Inject
    RxHttpClient httpClient

    def "GET /health should respond OK"() {
        when:
        def response = httpClient.toBlocking().exchange("/health")

        then:
        response.status() == HttpStatus.OK
    }
}