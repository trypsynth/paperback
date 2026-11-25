package dev.paperback.android.ui.state

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class DriveBookTransferTest {
	@Test
	fun wifiOnlyWaitsOnMobileData() {
		assertFalse(permitsBookTransfer(wifi = false, mobileData = false))
		assertTrue(permitsBookTransfer(wifi = true, mobileData = false))
	}

	@Test
	fun allowingMobileDataStillAllowsWifi() {
		assertTrue(permitsBookTransfer(wifi = false, mobileData = true))
		assertTrue(permitsBookTransfer(wifi = true, mobileData = true))
	}
}
