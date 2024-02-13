/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class edu_wpi_first_net_WPINetJNI */

#ifndef _Included_edu_wpi_first_net_WPINetJNI
#define _Included_edu_wpi_first_net_WPINetJNI
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    addPortForwarder
 * Signature: (ILjava/lang/String;I)V
 */
JNIEXPORT void JNICALL Java_edu_wpi_first_net_WPINetJNI_addPortForwarder
  (JNIEnv *, jclass, jint, jstring, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    removePortForwarder
 * Signature: (I)V
 */
JNIEXPORT void JNICALL Java_edu_wpi_first_net_WPINetJNI_removePortForwarder
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    createMulticastServiceAnnouncer
 * Signature: (Ljava/lang/String;Ljava/lang/String;I[Ljava/lang/String;[Ljava/lang/String;)I
 */
JNIEXPORT jint JNICALL Java_edu_wpi_first_net_WPINetJNI_createMulticastServiceAnnouncer
  (JNIEnv *, jclass, jstring, jstring, jint, jobjectArray, jobjectArray);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    freeMulticastServiceAnnouncer
 * Signature: (I)V
 */
JNIEXPORT void JNICALL Java_edu_wpi_first_net_WPINetJNI_freeMulticastServiceAnnouncer
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    startMulticastServiceAnnouncer
 * Signature: (I)V
 */
JNIEXPORT void JNICALL Java_edu_wpi_first_net_WPINetJNI_startMulticastServiceAnnouncer
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    stopMulticastServiceAnnouncer
 * Signature: (I)V
 */
JNIEXPORT void JNICALL Java_edu_wpi_first_net_WPINetJNI_stopMulticastServiceAnnouncer
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    getMulticastServiceAnnouncerHasImplementation
 * Signature: (I)Z
 */
JNIEXPORT jboolean JNICALL Java_edu_wpi_first_net_WPINetJNI_getMulticastServiceAnnouncerHasImplementation
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    createMulticastServiceResolver
 * Signature: (Ljava/lang/String;)I
 */
JNIEXPORT jint JNICALL Java_edu_wpi_first_net_WPINetJNI_createMulticastServiceResolver
  (JNIEnv *, jclass, jstring);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    freeMulticastServiceResolver
 * Signature: (I)V
 */
JNIEXPORT void JNICALL Java_edu_wpi_first_net_WPINetJNI_freeMulticastServiceResolver
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    startMulticastServiceResolver
 * Signature: (I)V
 */
JNIEXPORT void JNICALL Java_edu_wpi_first_net_WPINetJNI_startMulticastServiceResolver
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    stopMulticastServiceResolver
 * Signature: (I)V
 */
JNIEXPORT void JNICALL Java_edu_wpi_first_net_WPINetJNI_stopMulticastServiceResolver
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    getMulticastServiceResolverHasImplementation
 * Signature: (I)Z
 */
JNIEXPORT jboolean JNICALL Java_edu_wpi_first_net_WPINetJNI_getMulticastServiceResolverHasImplementation
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    getMulticastServiceResolverEventHandle
 * Signature: (I)I
 */
JNIEXPORT jint JNICALL Java_edu_wpi_first_net_WPINetJNI_getMulticastServiceResolverEventHandle
  (JNIEnv *, jclass, jint);

/*
 * Class:     edu_wpi_first_net_WPINetJNI
 * Method:    getMulticastServiceResolverData
 * Signature: (I)[Ledu/wpi/first/net/ServiceData;
 */
JNIEXPORT jobjectArray JNICALL Java_edu_wpi_first_net_WPINetJNI_getMulticastServiceResolverData
  (JNIEnv *, jclass, jint);

#ifdef __cplusplus
}
#endif
#endif