/**
 * 
 */
package smartCardProject;

import javacard.framework.Applet;
import javacard.framework.ISO7816;
import javacard.framework.ISOException;
import javacard.framework.APDU;
import javacard.framework.Util;
import java.security.KeyPairGenerator;
import java.security.NoSuchAlgorithmException;
import java.security.KeyPair;


public class SmartCardProject extends Applet {
	
	private final static byte[] hello=
	{0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x72, 0x6f, 0x62, 0x65, 0x72, 0x74};
	
	private java.security.Key publicRSAKey = null;
	private java.security.Key privateRSAKey = null;
	
	public static void install(byte[] buffer, short offset, byte length) 
	
	{
		// GP-compliant JavaCard applet registration
		SmartCardProject smartCardProject = new SmartCardProject();
		smartCardProject.register();
		try
		{
			KeyPairGenerator kpg = KeyPairGenerator.getInstance("RSA");
			kpg.initialize(512);
			KeyPair kp = kpg.generateKeyPair();
			smartCardProject.publicRSAKey = kp.getPublic();
			smartCardProject.privateRSAKey = kp.getPrivate();
		}
		catch (NoSuchAlgorithmException e)
		{
		}
		
	}

	public void process(APDU apdu) {
		// Good practice: Return 9000 on SELECT
		if (selectingApplet()) {
			return;
		}

		byte[] buf = apdu.getBuffer();
		switch (buf[ISO7816.OFFSET_INS]) {
		case (byte) 0x40:
			Util.arrayCopy(hello, (byte)0, buf, ISO7816.OFFSET_CDATA, (byte)12);
				apdu.setOutgoingAndSend(ISO7816.OFFSET_CDATA, (byte)12);
		
			break;
		default:
			// good practice: If you don't know the INStruction, say so:
			ISOException.throwIt(ISO7816.SW_INS_NOT_SUPPORTED);
		}
	}
}
