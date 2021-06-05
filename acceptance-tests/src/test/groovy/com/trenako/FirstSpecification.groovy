package com.trenako

import io.micronaut.test.extensions.spock.annotation.MicronautTest
import spock.lang.Specification

@MicronautTest
class FirstSpecification extends Specification {
    void "test it works"() {
        expect:
        1 == 1
    }
}