# GB/T 28181-2022 SIP SERVER

  ## 1. Features
  
  ### 1.1. Transport
    - ✅ 1.1.1. UDP sip server
    - ✅ 1.1.2. TCP sip server
    - ✅ 1.1.3. HTTP api server
  
  ### 1.2. Store
    - ✅ 1.2.1. Local memory
    - 📝 1.2.2. Redis
    - 📝 1.2.3. PostgreSQL
  
  ### 1.3. GB/T 28181-2016
    - ✅ 1.2.3.1.  Handle Register
    - ✅ 1.2.3.2.  Handle Unregister
    - ✅ 1.2.3.3.  Query DeviceStatus
    - 📝 1.2.3.4.  Query DeviceInfo
    - 📝 1.2.3.5.  Query Catelog
    - ✅ 1.2.3.6.  Handle Keepalive
    - ✅ 1.2.3.7.  Send Invite
    - ✅ 1.2.3.8.  Send Bye
    - 📝 1.2.3.9.  Send PTZ
    - 📝 1.2.3.10. Playback with fast or slow speed
    - 📝 1.2.3.11. Download playback with fast or slow speed
  
  ### 1.4. GB/T 28181-2022
    - ⬜ TODO
  
    
  ### 1.5. GB35114-2017
    - ⬜ TODO
  
  
  ## 2. Log
  ```log
  2024-08-20 15:18:42.3948133  INFO ThreadId(01) gbt_sip_server::utils::log: 50: start services
  ╔══════════════════════════════════════════════════════════╗
  ║          ┌─┐┌┐┌┬┐  ┌─┐┬┌─┐  ┌─┐┌─┐┬─┐┬  ┬┌─┐┬─┐          ║
  ║          │ ┬├┴┐│   └─┐│├─┘  └─┐├┤ ├┬┘└┐┌┘├┤ ├┬┘          ║
  ║          └─┘└─┘┴   └─┘┴┴    └─┘└─┘┴└─ └┘ └─┘┴└─          ║
  ║══════════════════════════════════════════════════════════║
  ║                                                          ║
  ║ git: https://github.com:FutureOrientedGB/gbt_sip_server  ║
  ║                                                          ║
  ║ version: e0ad50e.20240820.141228                         ║
  ║                                                          ║
  ║ store_engine: memory                                     ║
  ║ store_url: memory://main?total=16g                       ║
  ║ user_agent: gbt_sip_server                               ║
  ║ host: 0.0.0.0                                            ║
  ║ sip_ip: 192.168.3.139                                    ║
  ║ sip_port: 5060                                           ║
  ║ http_port: 8080                                          ║
  ║ sip_domain: 3402000000                                   ║
  ║ sip_id: 34020000002000000001                             ║
  ║ sip_password: d383cf85b0e8ce0b                           ║
  ║ sip_algorithm: md5                                       ║
  ║ sip_nonce: f89d0eaccaf1c90453e2f84688ec800f05            ║
  ║ sip_realm: gbt@future_oriented.com                       ║
  ║ socket_recv_buffer_size: 65535                           ║
  ╚══════════════════════════════════════════════════════════╝
  2024-08-20 15:18:42.4149901  INFO ThreadId(01) gbt_sip_server::sip::server: 28: UdpSocket::bind(0.0.0.0:5060) ok
  2024-08-20 15:18:42.4154556  INFO ThreadId(01) gbt_sip_server::sip::server: 37: TcpListener::bind(0.0.0.0:5060) ok
  2024-08-20 15:18:42.4179965  INFO ThreadId(01) gbt_sip_server::http::server: 26: HttpServer::bind(0.0.0.0:8080) ok
  2024-08-20 15:18:42.4180422  INFO ThreadId(01) actix_server::builder: 272: starting 12 workers
  2024-08-20 15:18:42.4181583  INFO ThreadId(01) actix_server::server: 192: Tokio runtime found; starting in existing Tokio runtime
  2024-08-20 15:18:42.4181904  INFO ThreadId(01) actix_server::server: 197: starting service: "actix-web-service-0.0.0.0:8080", workers: 12, listening on: 0.0.0.0:8080
  
  
  2024-08-20 15:18:49.6882891  INFO ThreadId(02) gbt_sip_server::sip::handler: 66: ⮜⮜⮜⮜⮜ sip_rs::Request::try_from(192.168.3.139:63241) ok, amount: 447, request:
  REGISTER SIP/2.0 sip:44010200492000000001@4401020049
  Via: SIP/2.0/UDP 192.168.3.139:63241;rport;branch=z9hG4bK6f04c
  From: <sip:44010200492000400301@4401020049>;tag=000037c8
  To: <sip:44010200492000400301@4401020049>
  Contact: <sip:44010200492000400301@192.168.3.139:63241>
  Call-ID: 0000525E00000306@192.168.3.139
  CSeq: 1 REGISTER
  Max-Forwards: 70
  Expires: 3600
  X-GB-Ver: 3.0
  User-Agent: Happytime GB28181 Device V7.0
  Content-Length: 0
  
  
  2024-08-20 15:18:49.6886912  INFO ThreadId(02) gbt_sip_server::sip::utils::sock: 111: ⮞⮞⮞⮞⮞ UdpSocket::send_to(192.168.3.139:63241) ok, amount: 414, response:
  SIP/2.0 401 Unauthorized
  Via: SIP/2.0/UDP 192.168.3.139:63241;rport;branch=z9hG4bK6f04c
  From: <sip:44010200492000400301@4401020049>;tag=000037c8
  To: <sip:44010200492000400301@4401020049>;tag=c18915397a
  Call-ID: 0000525E00000306@192.168.3.139
  CSeq: 1 REGISTER
  WWW-Authenticate: Digest realm="gbt@future_oriented.com", nonce="f89d0eaccaf1c90453e2f84688ec800f05", opaque="", algorithm=MD5
  Content-Length: 0
  
  
  2024-08-20 15:18:50.2892785  INFO ThreadId(02) gbt_sip_server::sip::handler: 66: ⮜⮜⮜⮜⮜ sip_rs::Request::try_from(192.168.3.139:63241) ok, amount: 687, request:
  REGISTER SIP/2.0 sip:44010200492000000001@4401020049
  Via: SIP/2.0/UDP 192.168.3.139:63241;rport;branch=z9hG4bK6f04c
  From: <sip:44010200492000400301@4401020049>;tag=000037c8
  To: <sip:44010200492000400301@4401020049>
  Contact: <sip:44010200492000400301@192.168.3.139:63241>
  Call-ID: 0000525E00000306@192.168.3.139
  CSeq: 2 REGISTER
  Max-Forwards: 70
  Expires: 3600
  X-GB-Ver: 3.0
  User-Agent: Happytime GB28181 Device V7.0
  Authorization: Digest username="44010200492000400301",realm="gbt@future_oriented.com",nonce="f89d0eaccaf1c90453e2f84688ec800f05",response="ec26084d0d848dda1482ca9ad58a634a",uri="sip:44010200492000000001@4401020049",opaque="",algorithm=MD5
  Content-Length: 0
  
  
  2024-08-20 15:18:50.2907771  INFO ThreadId(02) gbt_sip_server::sip::utils::sock: 111: ⮞⮞⮞⮞⮞ UdpSocket::send_to(192.168.3.139:63241) ok, amount: 276, response:
  SIP/2.0 200 OK
  Via: SIP/2.0/UDP 192.168.3.139:63241;rport;branch=z9hG4bK6f04c
  From: <sip:44010200492000400301@4401020049>;tag=000037c8
  To: <sip:44010200492000400301@4401020049>;tag=0eae2ec0f3
  Call-ID: 0000525E00000306@192.168.3.139
  CSeq: 2 REGISTER
  Content-Length: 0
  
  
  2024-08-20 15:18:50.2925037  INFO ThreadId(02) gbt_sip_server::sip::utils::sock: 111: ⮞⮞⮞⮞⮞ UdpSocket::send_to(192.168.3.139:63241) ok, amount: 542, request:
  MESSAGE sip:44010200492000400301@3402000000 SIP/2.0
  Via: SIP/2.0/UDP 192.168.3.139:5060;rport;branch=z9hG4bK6f04c
  Max-Forwards: 70
  From: <sip:34020000002000000001@3402000000>;tag=3fe6aebaf9
  To: <sip:44010200492000400301@3402000000>;tag=117ec6a30c
  Call-ID: 5891C49C3F594284AAF5E50F9892BDEA@192.168.3.139:5060
  CSeq: 1 MESSAGE
  User-Agent: gbt_sip_server e0ad50e.20240820.141228
  Content-Length: 136
  
  <?xml version="1.0" encoding="UTF-8"?><Query><CmdType>DeviceStatus</CmdType><SN>1</SN><DeviceID>44010200492000400301</DeviceID></Query>
  
  
  2024-08-20 15:18:50.2933788  INFO ThreadId(02) gbt_sip_server::sip::handler: 156: ⮜⮜⮜⮜⮜ sip_rs::Response::try_from(192.168.3.139:63241) ok, amount: 340, response:
  SIP/2.0 200 OK
  Via: SIP/2.0/UDP 192.168.3.139:5060;rport;branch=z9hG4bK6f04c
  From: <sip:34020000002000000001@3402000000>;tag=3fe6aebaf9
  To: <sip:44010200492000400301@3402000000>;tag=117ec6a30c
  Call-ID: 5891C49C3F594284AAF5E50F9892BDEA@192.168.3.139:5060
  CSeq: 1 MESSAGE
  User-Agent: Happytime GB28181 Device V7.0
  Content-Length: 0
  
  
  2024-08-20 15:18:52.9324212  INFO ThreadId(14) gbt_sip_server::sip::utils::sock: 111: ⮞⮞⮞⮞⮞ UdpSocket::send_to(192.168.3.139:63241) ok, amount: 837, request:
  INVITE sip:44010200492000400301@3402000000 SIP/2.0
  Via: SIP/2.0/UDP 192.168.3.139:5060;rport;branch=z9hG4bK6f04c
  Max-Forwards: 70
  From: <sip:34020000002000000001@3402000000>;tag=54a6ff3bb2
  To: <sip:44010200492000400301@3402000000>;tag=dffc40417e
  Contact: <sip:34020000002000000001@192.168.3.139:5060>
  Call-ID: 87CA42974A4542FBABA739D962039D58@192.168.3.139:5060
  CSeq: 2 INVITE
  Allow: INVITE, ACK, BYE, CANCEL, UPDATE, PRACK
  Supported: 100rel
  Subject: 44010200492000400301:0
  User-Agent: gbt_sip_server e0ad50e.20240820.141228
  Content-Type: application/sdp
  Content-Length: 246
  
  v=0
  o=44010200492000400301 0 0 IN IP4 127.0.0.1
  s=Play
  t=0 0
  m=video 12345 RTP/AVP 96 97 98 99
  c=IN IP4 127.0.0.1
  a=rtpmap:96 PS/90000
  a=rtpmap:97 MPEG4/90000
  a=rtpmap:98 H264/90000
  a=rtpmap:99 HEVC/90000
  a=recvonly
  a=streamMode:MAIN
  
  
  2024-08-20 15:18:52.9328525  INFO ThreadId(02) gbt_sip_server::sip::handler: 156: ⮜⮜⮜⮜⮜ sip_rs::Response::try_from(192.168.3.139:63241) ok, amount: 343, response:
  SIP/2.0 100 Trying
  Via: SIP/2.0/UDP 192.168.3.139:5060;rport;branch=z9hG4bK6f04c
  From: <sip:34020000002000000001@3402000000>;tag=54a6ff3bb2
  To: <sip:44010200492000400301@3402000000>;tag=dffc40417e
  Call-ID: 87CA42974A4542FBABA739D962039D58@192.168.3.139:5060
  CSeq: 2 INVITE
  User-Agent: Happytime GB28181 Device V7.0
  Content-Length: 0
  
  
  2024-08-20 15:18:52.933932  INFO ThreadId(02) gbt_sip_server::sip::handler: 156: ⮜⮜⮜⮜⮜ sip_rs::Response::try_from(192.168.3.139:63241) ok, amount: 609, response:
  SIP/2.0 200 OK
  Via: SIP/2.0/UDP 192.168.3.139:5060;rport;branch=z9hG4bK6f04c
  From: <sip:34020000002000000001@3402000000>;tag=54a6ff3bb2
  To: <sip:44010200492000400301@3402000000>;tag=dffc40417e
  Contact: <sip:44010200492000400301@3402000000>
  Call-ID: 87CA42974A4542FBABA739D962039D58@192.168.3.139:5060
  CSeq: 2 INVITE
  Max-Forwards: 70
  User-Agent: Happytime GB28181 Device V7.0
  Content-Type: application/sdp
  Content-Length: 171
  
  v=0
  o=44010200492000400301 0 0 IN IP4 192.168.3.139
  s=Play
  c=IN IP4 192.168.3.139
  t=0 0
  m=video 19002 RTP/AVP 96
  a=rtpmap:96 PS/90000
  a=sendonly
  f=v/2/5///a/1//1
  
  
  2024-08-20 15:18:52.9354295  INFO ThreadId(02) gbt_sip_server::sip::utils::sock: 111: ⮞⮞⮞⮞⮞ UdpSocket::send_to(192.168.3.139:63241) ok, amount: 329, request:
  ACK sip:44010200492000400301@3402000000 SIP/2.0
  Via: SIP/2.0/UDP 192.168.3.139:5060;rport;branch=z9hG4bK6f04c
  From: <sip:34020000002000000001@3402000000>;tag=54a6ff3bb2
  To: <sip:44010200492000400301@3402000000>;tag=734d500136
  Call-ID: 87CA42974A4542FBABA739D962039D58@192.168.3.139:5060
  CSeq: 2 INVITE
  Content-Length: 0
  
  
  2024-08-20 15:19:19.6885371  INFO ThreadId(02) gbt_sip_server::sip::handler: 66: ⮜⮜⮜⮜⮜ sip_rs::Request::try_from(192.168.3.139:63241) ok, amount: 572, request:
  MESSAGE SIP/2.0 sip:44010200492000000001@4401020049
  Via: SIP/2.0/UDP 192.168.3.139:63241;rport;branch=z9hG4bK00004823
  From: <sip:44010200492000400301@4401020049>;tag=000018be
  To: <sip:44010200492000000001@4401020049>
  Call-ID: 00004AE100006784@192.168.3.139
  CSeq: 1 MESSAGE
  Max-Forwards: 70
  User-Agent: Happytime GB28181 Device V7.0
  Content-Type: Application/MANSCDP+xml
  Content-Length: 170
  <?xml version="1.0" encoding="UTF-8"?>
  <Notify>
  <CmdType>Keepalive</CmdType>
  <SN>1</SN>
  <DeviceID>44010200492000400301</DeviceID>
  <Status>OK</Status>
  </Notify>
  
  
  2024-08-20 15:19:20.6627505  INFO ThreadId(15) gbt_sip_server::sip::utils::sock: 111: ⮞⮞⮞⮞⮞ UdpSocket::send_to(192.168.3.139:63241) ok, amount: 433, request:
  BYE sip:44010200492000400301@3402000000 SIP/2.0
  Via: SIP/2.0/UDP 192.168.3.139:5060;rport;branch=z9hG4bK6f04c
  Max-Forwards: 70
  From: <sip:34020000002000000001@3402000000>;tag=c92338c44c
  To: <sip:44010200492000400301@3402000000>;tag=6f2fda710e
  Contact: <sip:34020000002000000001@192.168.3.139:5060>
  Call-ID: 87CA42974A4542FBABA739D962039D58@192.168.3.139:5060
  CSeq: 2 BYE
  User-Agent: gbt_sip_server e0ad50e.20240820.141228
  
  
  2024-08-20 15:19:20.6650313  INFO ThreadId(02) gbt_sip_server::sip::handler: 156: ⮜⮜⮜⮜⮜ sip_rs::Response::try_from(192.168.3.139:63241) ok, amount: 336, response:
  SIP/2.0 200 OK
  Via: SIP/2.0/UDP 192.168.3.139:5060;rport;branch=z9hG4bK6f04c
  From: <sip:34020000002000000001@3402000000>;tag=c92338c44c
  To: <sip:44010200492000400301@3402000000>;tag=6f2fda710e
  Call-ID: 87CA42974A4542FBABA739D962039D58@192.168.3.139:5060
  CSeq: 2 BYE
  User-Agent: Happytime GB28181 Device V7.0
  Content-Length: 0
  
  
  ```

