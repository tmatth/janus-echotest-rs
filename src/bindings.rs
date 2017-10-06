/* automatically generated by rust-bindgen */

pub const JANUS_PLUGIN_API_VERSION: ::std::os::raw::c_uint = 8;
pub type gint = ::std::os::raw::c_int;
pub type gboolean = gint;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum json_type {
    JSON_OBJECT = 0,
    JSON_ARRAY = 1,
    JSON_STRING = 2,
    JSON_INTEGER = 3,
    JSON_REAL = 4,
    JSON_TRUE = 5,
    JSON_FALSE = 6,
    JSON_NULL = 7,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct json_t {
    pub type_: json_type,
    pub refcount: usize,
}
#[test]
fn bindgen_test_layout_json_t() {
    assert_eq!(::std::mem::size_of::<json_t>() , 16usize , concat ! (
               "Size of: " , stringify ! ( json_t ) ));
    assert_eq! (::std::mem::align_of::<json_t>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( json_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const json_t ) ) . type_ as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( json_t ) , "::" ,
                stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const json_t ) ) . refcount as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( json_t ) , "::" ,
                stringify ! ( refcount ) ));
}
impl Clone for json_t {
    fn clone(&self) -> Self { *self }
}
/// \brief Callbacks to contact the gateway
#[repr(C)]
#[derive(Debug, Copy)]
pub struct janus_callbacks {
    /// \brief Callback to push events/messages to a peer
    /// @note The Janus core increases the references to both the message and jsep
    /// json_t objects. This means that you'll have to decrease your own
    /// reference yourself with a \c json_decref after calling push_event.
    /// @param[in] handle The plugin/gateway session used for this peer
    /// @param[in] plugin The plugin instance that is sending the message/event
    /// @param[in] transaction The transaction identifier this message refers to
    /// @param[in] message The json_t object containing the JSON message
    /// @param[in] jsep The json_t object containing the JSEP type and the SDP attached to the message/event, if any (offer/answer)
    pub push_event: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                   *mut janus_plugin_session,
                                                               plugin:
                                                                   *mut janus_plugin,
                                                               transaction:
                                                                   *const ::std::os::raw::c_char,
                                                               message:
                                                                   *mut json_t,
                                                               jsep:
                                                                   *mut json_t)
                                              -> ::std::os::raw::c_int>,
    /// \brief Callback to relay RTP packets to a peer
    /// @param[in] handle The plugin/gateway session used for this peer
    /// @param[in] video Whether this is an audio or a video frame
    /// @param[in] buf The packet data (buffer)
    /// @param[in] len The buffer lenght
    pub relay_rtp: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                  *mut janus_plugin_session,
                                                              video:
                                                                  ::std::os::raw::c_int,
                                                              buf:
                                                                  *mut ::std::os::raw::c_char,
                                                              len:
                                                                  ::std::os::raw::c_int)>,
    /// \brief Callback to relay RTCP messages to a peer
    /// @param[in] handle The plugin/gateway session that will be used for this peer
    /// @param[in] video Whether this is related to an audio or a video stream
    /// @param[in] buf The message data (buffer)
    /// @param[in] len The buffer lenght
    pub relay_rtcp: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                   *mut janus_plugin_session,
                                                               video:
                                                                   ::std::os::raw::c_int,
                                                               buf:
                                                                   *mut ::std::os::raw::c_char,
                                                               len:
                                                                   ::std::os::raw::c_int)>,
    /// \brief Callback to relay SCTP/DataChannel messages to a peer
    /// @param[in] handle The plugin/gateway session that will be used for this peer
    /// @param[in] buf The message data (buffer)
    /// @param[in] len The buffer lenght
    pub relay_data: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                   *mut janus_plugin_session,
                                                               buf:
                                                                   *mut ::std::os::raw::c_char,
                                                               len:
                                                                   ::std::os::raw::c_int)>,
    /// \brief Callback to ask the core to close a WebRTC PeerConnection
    /// \note A call to this method will result in the core invoking the hangup_media
    /// callback on this plugin when done
    /// @param[in] handle The plugin/gateway session that the PeerConnection is related to
    pub close_pc: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                 *mut janus_plugin_session)>,
    /// \brief Callback to ask the core to get rid of a plugin/gateway session
    /// \note A call to this method will result in the core invoking the destroy_session
    /// callback on this plugin when done
    /// @param[in] handle The plugin/gateway session to get rid of
    pub end_session: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                    *mut janus_plugin_session)>,
    /// \brief Callback to check whether the event handlers mechanism is enabled
    /// @returns TRUE if it is, FALSE if it isn't (which means notify_event should NOT be called)
    pub events_is_enabled: ::std::option::Option<unsafe extern "C" fn()
                                                     -> gboolean>,
    /// \brief Callback to notify an event to the registered and subscribed event handlers
    /// \note Don't unref the event object, the core will do that for you
    /// @param[in] plugin The plugin originating the event
    /// @param[in] handle The plugin/gateway session originating the event, if any
    /// @param[in] event The event to notify as a Jansson json_t object
    pub notify_event: ::std::option::Option<unsafe extern "C" fn(plugin:
                                                                     *mut janus_plugin,
                                                                 handle:
                                                                     *mut janus_plugin_session,
                                                                 event:
                                                                     *mut json_t)>,
}
#[test]
fn bindgen_test_layout_janus_callbacks() {
    assert_eq!(::std::mem::size_of::<janus_callbacks>() , 64usize , concat ! (
               "Size of: " , stringify ! ( janus_callbacks ) ));
    assert_eq! (::std::mem::align_of::<janus_callbacks>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( janus_callbacks ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_callbacks ) ) . push_event as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_callbacks ) ,
                "::" , stringify ! ( push_event ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_callbacks ) ) . relay_rtp as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_callbacks ) ,
                "::" , stringify ! ( relay_rtp ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_callbacks ) ) . relay_rtcp as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_callbacks ) ,
                "::" , stringify ! ( relay_rtcp ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_callbacks ) ) . relay_data as *
                const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_callbacks ) ,
                "::" , stringify ! ( relay_data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_callbacks ) ) . close_pc as * const
                _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_callbacks ) ,
                "::" , stringify ! ( close_pc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_callbacks ) ) . end_session as *
                const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_callbacks ) ,
                "::" , stringify ! ( end_session ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_callbacks ) ) . events_is_enabled
                as * const _ as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_callbacks ) ,
                "::" , stringify ! ( events_is_enabled ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_callbacks ) ) . notify_event as *
                const _ as usize } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_callbacks ) ,
                "::" , stringify ! ( notify_event ) ));
}
impl Clone for janus_callbacks {
    fn clone(&self) -> Self { *self }
}
/// \brief The plugin session and callbacks interface
#[repr(C)]
#[derive(Debug, Copy)]
pub struct janus_plugin {
    /// \brief Plugin initialization/constructor
    /// @param[in] callback The callback instance the plugin can use to contact the gateway
    /// @param[in] config_path Path of the folder where the configuration for this plugin can be found
    /// @returns 0 in case of success, a negative integer in case of error
    pub init: ::std::option::Option<unsafe extern "C" fn(callback:
                                                             *mut janus_callbacks,
                                                         config_path:
                                                             *const ::std::os::raw::c_char)
                                        -> ::std::os::raw::c_int>,
    /// \brief Plugin deinitialization/destructor
    pub destroy: ::std::option::Option<unsafe extern "C" fn()>,
    /// \brief Informative method to request the API version this plugin was compiled against
    /// \note This was added in version 0.0.7 of the gateway, to address changes
    /// to the API that might break existing plugin or the core itself. All
    /// plugins MUST implement this method and return JANUS_PLUGIN_API_VERSION
    /// to make this work, or they will be rejected by the core. Do NOT try
    /// to launch a <= 0.0.7 plugin on a >= 0.0.7 gateway or it will crash.
    pub get_api_compatibility: ::std::option::Option<unsafe extern "C" fn()
                                                         ->
                                                             ::std::os::raw::c_int>,
    /// \brief Informative method to request the numeric version of the plugin
    pub get_version: ::std::option::Option<unsafe extern "C" fn()
                                               -> ::std::os::raw::c_int>,
    /// \brief Informative method to request the string version of the plugin
    pub get_version_string: ::std::option::Option<unsafe extern "C" fn()
                                                      ->
                                                          *const ::std::os::raw::c_char>,
    /// \brief Informative method to request a description of the plugin
    pub get_description: ::std::option::Option<unsafe extern "C" fn()
                                                   ->
                                                       *const ::std::os::raw::c_char>,
    /// \brief Informative method to request the name of the plugin
    pub get_name: ::std::option::Option<unsafe extern "C" fn()
                                            -> *const ::std::os::raw::c_char>,
    /// \brief Informative method to request the author of the plugin
    pub get_author: ::std::option::Option<unsafe extern "C" fn()
                                              ->
                                                  *const ::std::os::raw::c_char>,
    /// \brief Informative method to request the package name of the plugin (what will be used in web applications to refer to it)
    pub get_package: ::std::option::Option<unsafe extern "C" fn()
                                               ->
                                                   *const ::std::os::raw::c_char>,
    /// \brief Method to create a new session/handle for a peer
    /// @param[in] handle The plugin/gateway session that will be used for this peer
    /// @param[out] error An integer that may contain information about any error
    pub create_session: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                       *mut janus_plugin_session,
                                                                   error:
                                                                       *mut ::std::os::raw::c_int)>,
    /// \brief Method to handle an incoming message/request from a peer
    /// @param[in] handle The plugin/gateway session used for this peer
    /// @param[in] transaction The transaction identifier for this message/request
    /// @param[in] message The json_t object containing the message/request JSON
    /// @param[in] jsep The json_t object containing the JSEP type/SDP, if available
    /// @returns A janus_plugin_result instance that may contain a response (for immediate/synchronous replies), an ack
    /// (for asynchronously managed requests) or an error
    pub handle_message: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                       *mut janus_plugin_session,
                                                                   transaction:
                                                                       *mut ::std::os::raw::c_char,
                                                                   message:
                                                                       *mut json_t,
                                                                   jsep:
                                                                       *mut json_t)
                                                  ->
                                                      *mut janus_plugin_result>,
    /// \brief Callback to be notified when the associated PeerConnection is up and ready to be used
    /// @param[in] handle The plugin/gateway session used for this peer
    pub setup_media: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                    *mut janus_plugin_session)>,
    /// \brief Method to handle an incoming RTP packet from a peer
    /// @param[in] handle The plugin/gateway session used for this peer
    /// @param[in] video Whether this is an audio or a video frame
    /// @param[in] buf The packet data (buffer)
    /// @param[in] len The buffer lenght
    pub incoming_rtp: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                     *mut janus_plugin_session,
                                                                 video:
                                                                     ::std::os::raw::c_int,
                                                                 buf:
                                                                     *mut ::std::os::raw::c_char,
                                                                 len:
                                                                     ::std::os::raw::c_int)>,
    /// \brief Method to handle an incoming RTCP packet from a peer
    /// @param[in] handle The plugin/gateway session used for this peer
    /// @param[in] video Whether this is related to an audio or a video stream
    /// @param[in] buf The message data (buffer)
    /// @param[in] len The buffer lenght
    pub incoming_rtcp: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                      *mut janus_plugin_session,
                                                                  video:
                                                                      ::std::os::raw::c_int,
                                                                  buf:
                                                                      *mut ::std::os::raw::c_char,
                                                                  len:
                                                                      ::std::os::raw::c_int)>,
    /// \brief Method to handle incoming SCTP/DataChannel data from a peer (text only, for the moment)
    /// \note We currently only support text data, binary data will follow... please also notice that
    /// DataChannels send unterminated strings, so you'll have to terminate them with a \0 yourself to
    /// use them.
    /// @param[in] handle The plugin/gateway session used for this peer
    /// @param[in] buf The message data (buffer)
    /// @param[in] len The buffer lenght
    pub incoming_data: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                      *mut janus_plugin_session,
                                                                  buf:
                                                                      *mut ::std::os::raw::c_char,
                                                                  len:
                                                                      ::std::os::raw::c_int)>,
    /// \brief Method to be notified by the core when too many NACKs have
    /// been received or sent by Janus, and so a slow or potentially
    /// unreliable network is to be expected for this peer
    /// \note Beware that this callback may be called more than once in a row,
    /// (even though never more than once per second), until things go better for that
    /// PeerConnection. You may or may not want to handle this callback and
    /// act on it, considering you can get bandwidth information from REMB
    /// feedback sent by the peer if the browser supports it. Besides, your
    /// plugin may not have access to encoder related settings to slow down
    /// or decreae the bitrate if required after the callback is called.
    /// Nevertheless, it can be useful for debugging, or for informing your
    /// users about potential issues that may be happening media-wise.
    /// @param[in] handle The plugin/gateway session used for this peer
    /// @param[in] uplink Whether this is related to the uplink (Janus to peer)
    /// or downlink (peer to Janus)
    /// @param[in] video Whether this is related to an audio or a video stream
    pub slow_link: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                  *mut janus_plugin_session,
                                                              uplink:
                                                                  ::std::os::raw::c_int,
                                                              video:
                                                                  ::std::os::raw::c_int)>,
    /// \brief Callback to be notified about DTLS alerts from a peer (i.e., the PeerConnection is not valid any more)
    /// @param[in] handle The plugin/gateway session used for this peer
    pub hangup_media: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                     *mut janus_plugin_session)>,
    /// \brief Method to destroy a session/handle for a peer
    /// @param[in] handle The plugin/gateway session used for this peer
    /// @param[out] error An integer that may contain information about any error
    pub destroy_session: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                        *mut janus_plugin_session,
                                                                    error:
                                                                        *mut ::std::os::raw::c_int)>,
    /// \brief Method to get plugin-specific info of a session/handle
    /// \note This was added in version 0.0.7 of the gateway. Janus assumes
    /// the string is always allocated, so don't return constants here
    /// @param[in] handle The plugin/gateway session used for this peer
    /// @returns A json_t object with the requested info
    pub query_session: ::std::option::Option<unsafe extern "C" fn(handle:
                                                                      *mut janus_plugin_session)
                                                 -> *mut json_t>,
}
#[test]
fn bindgen_test_layout_janus_plugin() {
    assert_eq!(::std::mem::size_of::<janus_plugin>() , 152usize , concat ! (
               "Size of: " , stringify ! ( janus_plugin ) ));
    assert_eq! (::std::mem::align_of::<janus_plugin>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( janus_plugin ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . init as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( init ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . destroy as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( destroy ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . get_api_compatibility
                as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( get_api_compatibility ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . get_version as * const
                _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( get_version ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . get_version_string as
                * const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( get_version_string ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . get_description as *
                const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( get_description ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . get_name as * const _
                as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( get_name ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . get_author as * const
                _ as usize } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( get_author ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . get_package as * const
                _ as usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( get_package ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . create_session as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( create_session ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . handle_message as *
                const _ as usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( handle_message ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . setup_media as * const
                _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( setup_media ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . incoming_rtp as *
                const _ as usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( incoming_rtp ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . incoming_rtcp as *
                const _ as usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( incoming_rtcp ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . incoming_data as *
                const _ as usize } , 112usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( incoming_data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . slow_link as * const _
                as usize } , 120usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( slow_link ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . hangup_media as *
                const _ as usize } , 128usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( hangup_media ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . destroy_session as *
                const _ as usize } , 136usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( destroy_session ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin ) ) . query_session as *
                const _ as usize } , 144usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin ) , "::" ,
                stringify ! ( query_session ) ));
}
impl Clone for janus_plugin {
    fn clone(&self) -> Self { *self }
}
/// \brief Plugin-Gateway session mapping
#[repr(C)]
#[derive(Debug, Copy)]
pub struct janus_plugin_session {
    /// \brief Opaque pointer to the gateway session
    pub gateway_handle: *mut ::std::os::raw::c_void,
    /// \brief Opaque pointer to the plugin session
    pub plugin_handle: *mut ::std::os::raw::c_void,
    pub _bitfield_1: u8,
    pub __bindgen_padding_0: [u8; 7usize],
}
#[test]
fn bindgen_test_layout_janus_plugin_session() {
    assert_eq!(::std::mem::size_of::<janus_plugin_session>() , 24usize ,
               concat ! ( "Size of: " , stringify ! ( janus_plugin_session )
               ));
    assert_eq! (::std::mem::align_of::<janus_plugin_session>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( janus_plugin_session ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin_session ) ) . gateway_handle
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin_session )
                , "::" , stringify ! ( gateway_handle ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin_session ) ) . plugin_handle
                as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin_session )
                , "::" , stringify ! ( plugin_handle ) ));
}
impl Clone for janus_plugin_session {
    fn clone(&self) -> Self { *self }
}
impl janus_plugin_session {
    #[inline]
    pub fn stopped(&self) -> ::std::os::raw::c_int {
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        let mask = 1u64 as u8;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_stopped(&mut self, val: ::std::os::raw::c_int) {
        let mask = 1u64 as u8;
        let val = val as u32 as u8;
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(&unit_field_val as *const _ as
                                                *const u8,
                                            &mut self._bitfield_1 as *mut _ as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>());
        }
    }
    #[inline]
    pub fn new_bitfield_1(stopped: ::std::os::raw::c_int) -> u8 {
        ({ 0 } | ((stopped as u32 as u8) << 0usize) & (1u64 as u8))
    }
}
/// \brief Janus plugin result
#[repr(C)]
#[derive(Debug, Copy)]
pub struct janus_plugin_result {
    /// \brief Result type
    pub type_: janus_plugin_result_type,
    /// \brief Text associated with this plugin result.
    /// @note This is ONLY used for JANUS_PLUGIN_OK_WAIT (to provide hints on
    /// why a request is being handled asynchronously) and JANUS_PLUGIN_ERROR
    /// (to provide a reason for the error). It is ignored for JANUS_PLUGIN_OK.
    /// Besides, it is NOT freed when destroying the janus_plugin_result instance,
    /// so if you allocated a string for that, you'll have to free it yourself.
    pub text: *const ::std::os::raw::c_char,
    /// \brief Result content
    /// @note This is ONLY used for JANUS_PLUGIN_OK, and is ignored otherwise.
    /// It MUST be a valid JSON payload (even when returning application
    /// level errors). Its reference is decremented automatically when destroying
    /// the janus_plugin_result instance, so if your plugin wants to re-use the
    /// same object for multiple responses, you jave to \c json_incref the object before
    /// passing it to the core, and \c json_decref it when you're done with it.
    pub content: *mut json_t,
}
#[test]
fn bindgen_test_layout_janus_plugin_result() {
    assert_eq!(::std::mem::size_of::<janus_plugin_result>() , 24usize , concat
               ! ( "Size of: " , stringify ! ( janus_plugin_result ) ));
    assert_eq! (::std::mem::align_of::<janus_plugin_result>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( janus_plugin_result ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin_result ) ) . type_ as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin_result ) ,
                "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin_result ) ) . text as * const
                _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin_result ) ,
                "::" , stringify ! ( text ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const janus_plugin_result ) ) . content as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( janus_plugin_result ) ,
                "::" , stringify ! ( content ) ));
}
impl Clone for janus_plugin_result {
    fn clone(&self) -> Self { *self }
}
#[repr(i32)]
/// @name Janus plugin results
/// @brief When a client sends a message to a plugin (e.g., a request or a
/// command) this is notified to the plugin through a handle_message()
/// callback. The plugin can then either handle the request immediately
/// and provide a response (synchronous approach) or decide to queue it
/// and process it later (asynchronous approach). In both cases the plugin
/// must return a janus_plugin_result instance to the core, that will allow
/// the client to: 1. know whether a response is immediately available or
/// it will be later on through notifications, and 2. what the actual content
/// of the result might be. Of course, notifications related to the
/// transaction may occur later on even for synchronous requests, if the
/// plugin was implemented with use cases that envisage this approach.
/// @note An error may be returned as well, but this would cause a core-level
/// error to be returned to the client. If you want to provide indications
/// about a failed operation for application-level reason, the correct
/// approach is to return a success with a plugin-specific payload describing
/// the error.
/// /
/// ///@{
/// /*! \brief Result types
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum janus_plugin_result_type {
    JANUS_PLUGIN_ERROR = -1,
    JANUS_PLUGIN_OK = 0,
    JANUS_PLUGIN_OK_WAIT = 1,
}
extern "C" {
    /// \brief Helper to quickly create a janus_plugin_result instance
/// @param[in] type The type of result
/// @param[in] text String to add to the result (for JANUS_PLUGIN_OK_WAIT or JANUS_PLUGIN_ERROR), if any
/// @param[in] content The json_t object with the content of the result, if any
/// @returns A valid janus_plugin_result instance, if successful, or NULL otherwise
    pub fn janus_plugin_result_new(type_: janus_plugin_result_type,
                                   text: *const ::std::os::raw::c_char,
                                   content: *mut json_t)
     -> *mut janus_plugin_result;
}
extern "C" {
    /// \brief Helper to quickly destroy a janus_plugin_result instance
/// @param[in] result The janus_plugin_result instance to destroy
/// @returns A valid janus_plugin_result instance, if successful, or NULL otherwise
    pub fn janus_plugin_result_destroy(result: *mut janus_plugin_result);
}
