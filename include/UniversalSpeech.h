#ifndef ____UNIVERSAL_SPEECH_H__
#define ____UNIVERSAL_SPEECH_H__
#if defined __WIN32 || defined __WIN64 || defined _WIN32 || defined _WIN64
#define ____WINDOWS____
#define export __declspec(dllexport) __cdecl
#include<windows.h> // for wchar_t
typedef wchar_t ____wchar_t;
#else
#error Platform currently unsupported
#endif
#ifdef __cplusplus
extern "C" {
#endif

/** Parameters used for functions speechSetValue, speechGetValue, speechSetString and speechGetString. */
enum speechparam {
/** The following few values are in groupes of 4 parameters. 
+ First one is used to get or set the current value for this parameter
+ Second and third one are used to retriev the minimum and the maximum values allowed for the parameter
+ Last one is used to query if the currently active engine, subengine, language and voice supports the parameter. If 0 is returned, then the parameter isn't supported. Any other value different than 0 means that the parameter is supported and thus can be get or set. A supported parameter may be read only and thus refuse any attend to change the value.
*/

/** Speech volume. smaller values usually means quieter, and bigger values mean louder. Often, 0 means completely silent.   */
SP_VOLUME, SP_VOLUME_MAX, SP_VOLUME_MIN,  SP_VOLUME_SUPPORTED,

/** Speech rate, a.k.a speed. Small valuers usually mean slower, and bigger values mean faster. No unit such as words per minute is enforced. */
SP_RATE, SP_RATE_MAX, SP_RATE_MIN, SP_RATE_SUPPORTED,

/** Voice pitch. Usually small values stands for lower pitch, bigger values for higher pitch.  */
SP_PITCH, SP_PITCH_MAX, SP_PITCH_MIN, SP_PITCH_SUPPORTED,

/** Voice inflexion, a.k.a pitch fluctuation or emotivity. Smaller values usually means a monotone speech like a robot, while bigger values means a voice which seems excited and which exagerate pitch elevation when interpreting punctuation. . */
SP_INFLEXION, SP_INFLEXION_MAX, SP_INFLEXION_MIN, SP_INFLEXION_SUPPORTED,

/** Pausing the speech while it is in progress. 0 means off, and any value different than 0 means on. */
SP_PAUSED, SP_PAUSE_SUPPORTED, 

/** Querying if a speech is currently in progress (read only parameter). 0 means no, any other value means yes. */
SP_BUSY, SP_BUSY_SUPPORTED, 

/** Wait for the speech to finish. Use speechGetValue with this parameter to wait indefinitely for the current speech to finish. Use speechSetValue with a number of milliseconds to wait at most the specified time for the current speech to finish. */
SP_WAIT, SP_WAIT_SUPPORTED,

/** Enable or disable specific OS speech engines that can in principle never fail, such as SAPI5 on windows. 0 means disabled, any other value means enabled. This parameter is disabled by default. */
SP_ENABLE_NATIVE_SPEECH = 0xFFFF, 

/** Voice specific to the current engine.
Some engines supports different voices. Screen readers usually do not provite that in their API, but multiple voices can be installed into OS specific engines such as SAPI5. This parameter is used to get or set the current voice (0-based index), or to query for voices names.
+ use speechGetValue with this parameter to query the currently selected voice, as a 0-based index
+ Use speechSetValue with this parameter to set the current voice, as a 0-based index
+ use speechGetString with this parameter + n to retriev the voice name of the nth 0-based index voice, i.e. speechGetString(SP_VOICE+5) to retriev the name of the 6th voice. You can iterate starting at 0 until you get NULL to discover how many voices the current engine has.
*/
SP_VOICE = 0x10000,

/** Same principle as voices but for languages. Some engines allow to change dynamically the speech language. Note that the list of available voices may change following a language change, depending on how the engine works. */
SP_LANGUAGE = 0x20000,

/** Same principle as voices and languages but for sub-engines. Some engines are in fact wrappers that can handle a collection of engines, such as speech dispatcher on linux. This parameter allow to select an engine out of an engine wrapper. In general, languages and voices available changes following a sub-engine change. */
SP_SUBENGINE = 0x30000,

/** Always the same principle, but this time for top-level engines. Being present on the list does not necessarily mean that the engine is available and working. Generally, you will let the user select one of the supported ones. 
By default, the system automatically select the first available and working engine, and switch automatically to another engine when the current one become unavailable.
When querying, it returns the actual current engine used as 0-based index. A return value of -1 means that the system didn't find any suitable engine to work with.
By setting the engine to a positive value, you force the system to use this engine, even if it is in fact inoperant.
You can restore the default behavior by setting the engine to -1.
*/
SP_ENGINE = 0x40000,

/** Engine availability
Still the same principle, but for engine availability. Query for example the parameter SP_ENGINE_AVAILABLE+3 to determine if 4th engine is currently available and working if you select it.
0 means unavailable, any other value mean available. 
Normally, if you loop for names with SP_ENGINE+n, you will get all engines names supported by universal speech, not necessarily those which are really ready to be used. Querying for actual availability allow you to filter the list presented to the user.
*/
SP_ENGINE_AVAILABLE = 0x50000,

/** Query if default automatic detection is used or not */
SP_AUTO_ENGINE = 0xFFFE,

/** Some engines or subengines may give access to additionnal specific parameters
Engine wrapper devloppers shouldn't use a parameter identifier below this value, to ensure that there wont be any conflict with a standard parameter (one of the above)
*/
SP_USER_PARAM = 0x1000000
};

/** Send a message to be spoken by the current engine, subengine, language and voice
All strings must be in unicode, i.e. use wchar_t, WCHAR, std::wstring
The second parameter controls if this message must interrupt the currently spoken text if any, or not. 
+ A value of 0 means false, i.e. this message is appended to the queue and will be spoken after all previous messages are finished. 
+ Any value different than 0 means true, i.e. if something is currently spoken, it is interrupted and this message is immediately spoken, clearing the queue of messages still waiting.
*/
export int speechSay (const ____wchar_t* str, int interrupt) ;

/** ANSI version of the function above, for those who don't support unicode. */
export int speechSayA (const char* str, int interrupt) ;

/** Send a message to be displayed on a braille display by the current engine, if supported
All strings must be in unicode, i.e. use wchar_t, WCHAR, std::wstring
*/
export int brailleDisplay (const ____wchar_t* str) ;

/** ANSI version of the function above, for those who don't support unicode. */
export int brailleDisplayA (const char* str) ;


/** Immediately stop speaking and clear the queue of pending messages */
export int speechStop (void) ;

/** Query for an integer parameter. Look at the list of all parameters in the enum above */
export int speechGetValue (int what) ;

/** Request an integer parameter change. Look at the list of all parameters in the enum above. */
export int speechSetValue (int what, int value) ;

/** Query for a string parameter. Look at the list of all parameters in the enum above.
All strings are returned in unicode and don't have to be free by the caller unless otherwise specified.
 */
export const ____wchar_t* speechGetString (int what) ;

/** Request a parameter change. Look at the list of all parameters in the enum above. 
All strings must be in unicode and don't have to be valid after the function has returned, unless otherwise specified.
*/
export int speechSetString (int what, const ____wchar_t* str) ;

/** ANSI version of the functions above, for those who don't support unicode. */
export const char* speechGetStringA (int what) ;
export int speechSetStringA (int what, const char* value) ;

#ifdef __cplusplus
} // extern "C"
#endif
#endif
