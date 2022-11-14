/**
 * 
 */
package smartCardProject;

import javacard.framework.Applet;
import javacard.framework.ISO7816;
import javacard.framework.ISOException;
import javacard.framework.APDU;
import javacard.framework.Util;
import javacard.security.KeyPair;
import javacard.security.KeyBuilder;


public class SmartCardProject extends Applet {
	
	private final static byte[] hello=
	{0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x72, 0x6f, 0x62, 0x65, 0x72, 0x74};
	
	private javacard.security.PublicKey publicRSAKey = null;
	private javacard.security.PrivateKey privateRSAKey = null;
	
	public static void install(byte[] buffer, short offset, byte length) 
	
	{
		// GP-compliant JavaCard applet registration
		SmartCardProject smartCardProject = new SmartCardProject();
		smartCardProject.register();
		
		KeyPair kpg = new KeyPair(KeyPair.ALG_RSA, KeyBuilder.LENGTH_RSA_512);
		smartCardProject.publicRSAKey = kpg.getPublic();
		smartCardProject.privateRSAKey = kpg.getPrivate();
		
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
