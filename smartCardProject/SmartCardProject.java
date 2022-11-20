package smartCardProject;

import javacard.framework.Applet;
import javacard.framework.ISO7816;
import javacard.framework.ISOException;
import javacard.framework.APDU;
import javacard.framework.Util;
import javacard.security.KeyPair;
import javacard.security.KeyBuilder;
import javacard.framework.OwnerPIN;

// use OwnerPIN for PIN code ?

public class SmartCardProject extends Applet
{
	/// Instructions list
	// Convention: use same digit for linked instructions
	// ie: 0x2n -> login, logout, change pin...
	private static final byte INST_HELLO = 0x10;
	private static final byte INST_AUTH = 0x20;
	
	
	public static final short DEFAULT_PIN_CODE = 0000;
	private static final short PIN_MAX_RETRIES = 5;
	private static final short PIN_SIZE = 4;
	
	private final static byte[] HELLO_STR =
	{0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x72, 0x6f, 0x62, 0x65, 0x72, 0x74};
	
	private final static byte[] OK_RESPONSE = {'O', 'K'};
	private final static byte[] KO_RESPONSE = {'K', 'O'};
	
	private javacard.security.PublicKey publicRSAKey = null;
	private javacard.security.PrivateKey privateRSAKey = null;
	private short pinCode = DEFAULT_PIN_CODE;
	private boolean cardConnected = false;
	
	private OwnerPIN ownerPin;
	
	private class APDUData
	{
		public APDU apdu;
		public byte[] apduBuffer;
		
		public APDUData(APDU apdu, byte[] apduBuffer)
		{
			this.apdu = apdu;
			this.apduBuffer = apduBuffer;
		}
	}
	
	public static void install(byte[] buffer, short offset, byte length)
	{
		// GP-compliant JavaCard applet registration
		SmartCardProject smartCardProject = new SmartCardProject();
		smartCardProject.register();
		
		KeyPair kpg = new KeyPair(KeyPair.ALG_RSA, KeyBuilder.LENGTH_RSA_512);
		smartCardProject.publicRSAKey = kpg.getPublic();
		smartCardProject.privateRSAKey = kpg.getPrivate();
		
	}
	
	SmartCardProject()
	{
		ownerPin = new OwnerPIN((byte)PIN_MAX_RETRIES, (byte)PIN_SIZE);
	}
	
	// TODO: use https://github.com/neonlzf/smartcard/blob/master/Code/OnCard/DES/RSA.java

	public void process(APDU apdu) {
		// Good practice: Return 9000 on SELECT
		if (selectingApplet()) {
			ISOException.throwIt(ISO7816.SW_NO_ERROR);
		}

		byte[] apduBuffer = apdu.getBuffer();
		
		// SELECT
		if ((apduBuffer[ISO7816.OFFSET_CLA] == 0) && (apduBuffer[ISO7816.OFFSET_INS] == (byte) 0xA4)) {
            		return;
        	}

        	if (apduBuffer[ISO7816.OFFSET_CLA] != 0x0) {
            		ISOException.throwIt(ISO7816.SW_CLA_NOT_SUPPORTED);
        	}
		
		// Retrieve command data
		short bytesLeft = Util.makeShort((byte) 0x00, apduBuffer[ISO7816.OFFSET_LC]);
        	if (bytesLeft != apdu.setIncomingAndReceive()) {
            		ISOException.throwIt(ISO7816.SW_WRONG_LENGTH);
        	}
		
		switch (apduBuffer[ISO7816.OFFSET_INS]) {
		case INST_HELLO:
			instHello(apdu);
			break;
		case INST_AUTH:
			instAuth(apdu);
			break;
		default:
			// good practice: If you don't know the INStruction, say so:
			ISOException.throwIt(ISO7816.SW_INS_NOT_SUPPORTED);
		}
	}
	
	/// Instructions methods
	
	private void instHello(APDU apdu)
	{
		sendAPDUResponse(apdu, HELLO_STR);
	}
	
	private void instAuth(APDU apdu) // TODO: error message more clear
	{
		if (ownerPin.isValidated())
		{
			sendAPDUResponse(apdu, OK_RESPONSE);
			return;
		}
		if (ownerPin.getTriesRemaining() == 0)
		{
			sendAPDUResponse(apdu, KO_RESPONSE);
			return;
		}
		
		byte[] apduBuffer = apdu.getBuffer();
		
		if (apduBuffer[ISO7816.OFFSET_LC] != PIN_SIZE) {
            		sendAPDUResponse(apdu, KO_RESPONSE);
			return;
        	}
        	
        	
        	// Code is correct
        	if (ownerPin.check(apduBuffer, ISO7816.OFFSET_CDATA, (byte)PIN_SIZE))
        	{
        		sendAPDUResponse(apdu, OK_RESPONSE);
			return;
        	}
        	
		sendAPDUResponse(apdu, KO_RESPONSE);
	}
	
	private void sendAPDUResponse(APDU apdu, byte[] response)
	{
		byte[] buf = apdu.getBuffer();
		Util.arrayCopy(response, (short)0, buf, ISO7816.OFFSET_CDATA, (short)response.length);
		apdu.setOutgoingAndSend(ISO7816.OFFSET_CDATA, (short)response.length);
	}
}
