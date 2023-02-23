use super::*;

/// A globally available set of GL functions.
///
/// Because of the [RwLock](std::sync::RwLock) wrapper, loading the functions
/// and calling them is fully synchronized across threads.
///
/// * Performing the loading is still `unsafe` (see [GlFns::load]).
/// * Calling the functions is still `unsafe` too.
///
/// See the top level crate docs for a usage example.
#[cfg(feature = "std")]
#[cfg_attr(docs_rs, doc(cfg(feature = "std")))]
pub static GL: std::sync::RwLock<GlFns> = std::sync::RwLock::new(BLANK_GL_FNS);

impl GlFns {
  /// Constructs a new, blank `GlFns` value directly on the heap.
  #[inline]
  #[must_use]
  #[cfg(feature = "alloc")]
  #[cfg_attr(docs_rs, doc(cfg(feature = "alloc")))]
  pub fn new_boxed() -> alloc::boxed::Box<Self> {
    use alloc::{boxed::Box, vec, vec::Vec};
    // this struct is usually *hundreds* of `usize` big,
    // so we do this heap dance to avoid stack strain.
    assert!(core::mem::size_of::<Self>() % core::mem::size_of::<Option<fn()>>() == 0);
    let len = core::mem::size_of::<Self>() / core::mem::size_of::<Option<fn()>>();
    let v: Vec<Option<fn()>> = vec![None; len];
    #[allow(clippy::type_complexity)]
    let b: Box<[Option<fn()>]> = v.into_boxed_slice();
    let ptr_slice: *mut [Option<fn()>] = Box::leak(b);
    let ptr_self: *mut Self = ptr_slice as *mut Self;
    let box_self: Box<Self> = unsafe { Box::from_raw(ptr_self) };
    box_self
  }

  /// Calls the user provided closure to fill as many of the GL function pointer
  /// fields as possible.
  ///
  /// ## Safety
  /// * Each `*const u8` passed to the user closure will point to the start of a
  ///   C string naming the GL function to load. Pass this to the appropriate OS
  ///   function for loading a GL function pointer and return what the OS gives
  ///   you from your closure.
  /// * If you return any other non-null pointer from your closure it can cause
  ///   arbitrary code execution errors, so don't.
  pub unsafe fn load(
    &mut self, mut user_loader: impl FnMut(*const u8) -> *const core::ffi::c_void,
  ) {
    use core::mem::transmute;
    let mut loader = |name| {
      let fn_p = user_loader(name);
      match fn_p as usize {
        // Note(Lokathor): these fn address values are "error" values on some
        // platforms, and should *not* be called.
        1 | 2 | 3 | 4 | usize::MAX => core::ptr::null(),
        _ => fn_p,
      }
    };
    self.glActiveShaderProgram = unsafe { transmute(loader("glActiveShaderProgram\0".as_ptr())) };
    self.glActiveTexture = unsafe { transmute(loader("glActiveTexture\0".as_ptr())) };
    self.glAttachShader = unsafe { transmute(loader("glAttachShader\0".as_ptr())) };
    self.glBeginConditionalRender =
      unsafe { transmute(loader("glBeginConditionalRender\0".as_ptr())) };
    self.glBeginQuery = unsafe { transmute(loader("glBeginQuery\0".as_ptr())) };
    self.glBeginQueryIndexed = unsafe { transmute(loader("glBeginQueryIndexed\0".as_ptr())) };
    self.glBeginTransformFeedback =
      unsafe { transmute(loader("glBeginTransformFeedback\0".as_ptr())) };
    self.glBindAttribLocation = unsafe { transmute(loader("glBindAttribLocation\0".as_ptr())) };
    self.glBindBuffer = unsafe { transmute(loader("glBindBuffer\0".as_ptr())) };
    self.glBindBufferBase = unsafe { transmute(loader("glBindBufferBase\0".as_ptr())) };
    self.glBindBufferRange = unsafe { transmute(loader("glBindBufferRange\0".as_ptr())) };
    self.glBindBuffersBase = unsafe { transmute(loader("glBindBuffersBase\0".as_ptr())) };
    self.glBindBuffersRange = unsafe { transmute(loader("glBindBuffersRange\0".as_ptr())) };
    self.glBindFragDataLocation = unsafe { transmute(loader("glBindFragDataLocation\0".as_ptr())) };
    self.glBindFragDataLocationIndexed =
      unsafe { transmute(loader("glBindFragDataLocationIndexed\0".as_ptr())) };
    self.glBindFramebuffer = unsafe { transmute(loader("glBindFramebuffer\0".as_ptr())) };
    self.glBindImageTexture = unsafe { transmute(loader("glBindImageTexture\0".as_ptr())) };
    self.glBindImageTextures = unsafe { transmute(loader("glBindImageTextures\0".as_ptr())) };
    self.glBindProgramPipeline = unsafe { transmute(loader("glBindProgramPipeline\0".as_ptr())) };
    self.glBindRenderbuffer = unsafe { transmute(loader("glBindRenderbuffer\0".as_ptr())) };
    self.glBindSampler = unsafe { transmute(loader("glBindSampler\0".as_ptr())) };
    self.glBindSamplers = unsafe { transmute(loader("glBindSamplers\0".as_ptr())) };
    self.glBindTexture = unsafe { transmute(loader("glBindTexture\0".as_ptr())) };
    self.glBindTextureUnit = unsafe { transmute(loader("glBindTextureUnit\0".as_ptr())) };
    self.glBindTextures = unsafe { transmute(loader("glBindTextures\0".as_ptr())) };
    self.glBindTransformFeedback =
      unsafe { transmute(loader("glBindTransformFeedback\0".as_ptr())) };
    self.glBindVertexArray = unsafe { transmute(loader("glBindVertexArray\0".as_ptr())) };
    self.glBindVertexBuffer = unsafe { transmute(loader("glBindVertexBuffer\0".as_ptr())) };
    self.glBindVertexBuffers = unsafe { transmute(loader("glBindVertexBuffers\0".as_ptr())) };
    self.glBlendBarrier = unsafe { transmute(loader("glBlendBarrier\0".as_ptr())) };
    self.glBlendColor = unsafe { transmute(loader("glBlendColor\0".as_ptr())) };
    self.glBlendEquation = unsafe { transmute(loader("glBlendEquation\0".as_ptr())) };
    self.glBlendEquationSeparate =
      unsafe { transmute(loader("glBlendEquationSeparate\0".as_ptr())) };
    self.glBlendEquationSeparatei =
      unsafe { transmute(loader("glBlendEquationSeparatei\0".as_ptr())) };
    self.glBlendEquationi = unsafe { transmute(loader("glBlendEquationi\0".as_ptr())) };
    self.glBlendFunc = unsafe { transmute(loader("glBlendFunc\0".as_ptr())) };
    self.glBlendFuncSeparate = unsafe { transmute(loader("glBlendFuncSeparate\0".as_ptr())) };
    self.glBlendFuncSeparatei = unsafe { transmute(loader("glBlendFuncSeparatei\0".as_ptr())) };
    self.glBlendFunci = unsafe { transmute(loader("glBlendFunci\0".as_ptr())) };
    self.glBlitFramebuffer = unsafe { transmute(loader("glBlitFramebuffer\0".as_ptr())) };
    self.glBlitNamedFramebuffer = unsafe { transmute(loader("glBlitNamedFramebuffer\0".as_ptr())) };
    self.glBufferData = unsafe { transmute(loader("glBufferData\0".as_ptr())) };
    self.glBufferStorage = unsafe { transmute(loader("glBufferStorage\0".as_ptr())) };
    self.glBufferSubData = unsafe { transmute(loader("glBufferSubData\0".as_ptr())) };
    self.glCheckFramebufferStatus =
      unsafe { transmute(loader("glCheckFramebufferStatus\0".as_ptr())) };
    self.glCheckNamedFramebufferStatus =
      unsafe { transmute(loader("glCheckNamedFramebufferStatus\0".as_ptr())) };
    self.glClampColor = unsafe { transmute(loader("glClampColor\0".as_ptr())) };
    self.glClear = unsafe { transmute(loader("glClear\0".as_ptr())) };
    self.glClearBufferData = unsafe { transmute(loader("glClearBufferData\0".as_ptr())) };
    self.glClearBufferSubData = unsafe { transmute(loader("glClearBufferSubData\0".as_ptr())) };
    self.glClearBufferfi = unsafe { transmute(loader("glClearBufferfi\0".as_ptr())) };
    self.glClearBufferfv = unsafe { transmute(loader("glClearBufferfv\0".as_ptr())) };
    self.glClearBufferiv = unsafe { transmute(loader("glClearBufferiv\0".as_ptr())) };
    self.glClearBufferuiv = unsafe { transmute(loader("glClearBufferuiv\0".as_ptr())) };
    self.glClearColor = unsafe { transmute(loader("glClearColor\0".as_ptr())) };
    self.glClearDepth = unsafe { transmute(loader("glClearDepth\0".as_ptr())) };
    self.glClearDepthf = unsafe { transmute(loader("glClearDepthf\0".as_ptr())) };
    self.glClearNamedBufferData = unsafe { transmute(loader("glClearNamedBufferData\0".as_ptr())) };
    self.glClearNamedBufferSubData =
      unsafe { transmute(loader("glClearNamedBufferSubData\0".as_ptr())) };
    self.glClearNamedFramebufferfi =
      unsafe { transmute(loader("glClearNamedFramebufferfi\0".as_ptr())) };
    self.glClearNamedFramebufferfv =
      unsafe { transmute(loader("glClearNamedFramebufferfv\0".as_ptr())) };
    self.glClearNamedFramebufferiv =
      unsafe { transmute(loader("glClearNamedFramebufferiv\0".as_ptr())) };
    self.glClearNamedFramebufferuiv =
      unsafe { transmute(loader("glClearNamedFramebufferuiv\0".as_ptr())) };
    self.glClearStencil = unsafe { transmute(loader("glClearStencil\0".as_ptr())) };
    self.glClearTexImage = unsafe { transmute(loader("glClearTexImage\0".as_ptr())) };
    self.glClearTexSubImage = unsafe { transmute(loader("glClearTexSubImage\0".as_ptr())) };
    self.glClientWaitSync = unsafe { transmute(loader("glClientWaitSync\0".as_ptr())) };
    self.glClipControl = unsafe { transmute(loader("glClipControl\0".as_ptr())) };
    self.glColorMask = unsafe { transmute(loader("glColorMask\0".as_ptr())) };
    self.glColorMaski = unsafe { transmute(loader("glColorMaski\0".as_ptr())) };
    self.glColorP3ui = unsafe { transmute(loader("glColorP3ui\0".as_ptr())) };
    self.glColorP3uiv = unsafe { transmute(loader("glColorP3uiv\0".as_ptr())) };
    self.glColorP4ui = unsafe { transmute(loader("glColorP4ui\0".as_ptr())) };
    self.glColorP4uiv = unsafe { transmute(loader("glColorP4uiv\0".as_ptr())) };
    self.glCompileShader = unsafe { transmute(loader("glCompileShader\0".as_ptr())) };
    self.glCompressedTexImage1D = unsafe { transmute(loader("glCompressedTexImage1D\0".as_ptr())) };
    self.glCompressedTexImage2D = unsafe { transmute(loader("glCompressedTexImage2D\0".as_ptr())) };
    self.glCompressedTexImage3D = unsafe { transmute(loader("glCompressedTexImage3D\0".as_ptr())) };
    self.glCompressedTexSubImage1D =
      unsafe { transmute(loader("glCompressedTexSubImage1D\0".as_ptr())) };
    self.glCompressedTexSubImage2D =
      unsafe { transmute(loader("glCompressedTexSubImage2D\0".as_ptr())) };
    self.glCompressedTexSubImage3D =
      unsafe { transmute(loader("glCompressedTexSubImage3D\0".as_ptr())) };
    self.glCompressedTextureSubImage1D =
      unsafe { transmute(loader("glCompressedTextureSubImage1D\0".as_ptr())) };
    self.glCompressedTextureSubImage2D =
      unsafe { transmute(loader("glCompressedTextureSubImage2D\0".as_ptr())) };
    self.glCompressedTextureSubImage3D =
      unsafe { transmute(loader("glCompressedTextureSubImage3D\0".as_ptr())) };
    self.glCopyBufferSubData = unsafe { transmute(loader("glCopyBufferSubData\0".as_ptr())) };
    self.glCopyImageSubData = unsafe { transmute(loader("glCopyImageSubData\0".as_ptr())) };
    self.glCopyNamedBufferSubData =
      unsafe { transmute(loader("glCopyNamedBufferSubData\0".as_ptr())) };
    self.glCopyTexImage1D = unsafe { transmute(loader("glCopyTexImage1D\0".as_ptr())) };
    self.glCopyTexImage2D = unsafe { transmute(loader("glCopyTexImage2D\0".as_ptr())) };
    self.glCopyTexSubImage1D = unsafe { transmute(loader("glCopyTexSubImage1D\0".as_ptr())) };
    self.glCopyTexSubImage2D = unsafe { transmute(loader("glCopyTexSubImage2D\0".as_ptr())) };
    self.glCopyTexSubImage3D = unsafe { transmute(loader("glCopyTexSubImage3D\0".as_ptr())) };
    self.glCopyTextureSubImage1D =
      unsafe { transmute(loader("glCopyTextureSubImage1D\0".as_ptr())) };
    self.glCopyTextureSubImage2D =
      unsafe { transmute(loader("glCopyTextureSubImage2D\0".as_ptr())) };
    self.glCopyTextureSubImage3D =
      unsafe { transmute(loader("glCopyTextureSubImage3D\0".as_ptr())) };
    self.glCreateBuffers = unsafe { transmute(loader("glCreateBuffers\0".as_ptr())) };
    self.glCreateFramebuffers = unsafe { transmute(loader("glCreateFramebuffers\0".as_ptr())) };
    self.glCreateProgram = unsafe { transmute(loader("glCreateProgram\0".as_ptr())) };
    self.glCreateProgramPipelines =
      unsafe { transmute(loader("glCreateProgramPipelines\0".as_ptr())) };
    self.glCreateQueries = unsafe { transmute(loader("glCreateQueries\0".as_ptr())) };
    self.glCreateRenderbuffers = unsafe { transmute(loader("glCreateRenderbuffers\0".as_ptr())) };
    self.glCreateSamplers = unsafe { transmute(loader("glCreateSamplers\0".as_ptr())) };
    self.glCreateShader = unsafe { transmute(loader("glCreateShader\0".as_ptr())) };
    self.glCreateShaderProgramv = unsafe { transmute(loader("glCreateShaderProgramv\0".as_ptr())) };
    self.glCreateTextures = unsafe { transmute(loader("glCreateTextures\0".as_ptr())) };
    self.glCreateTransformFeedbacks =
      unsafe { transmute(loader("glCreateTransformFeedbacks\0".as_ptr())) };
    self.glCreateVertexArrays = unsafe { transmute(loader("glCreateVertexArrays\0".as_ptr())) };
    self.glCullFace = unsafe { transmute(loader("glCullFace\0".as_ptr())) };
    self.glDebugMessageCallback = unsafe { transmute(loader("glDebugMessageCallback\0".as_ptr())) };
    self.glDebugMessageCallbackKHR =
      unsafe { transmute(loader("glDebugMessageCallbackKHR\0".as_ptr())) };
    self.glDebugMessageControl = unsafe { transmute(loader("glDebugMessageControl\0".as_ptr())) };
    self.glDebugMessageControlKHR =
      unsafe { transmute(loader("glDebugMessageControlKHR\0".as_ptr())) };
    self.glDebugMessageInsert = unsafe { transmute(loader("glDebugMessageInsert\0".as_ptr())) };
    self.glDebugMessageInsertKHR =
      unsafe { transmute(loader("glDebugMessageInsertKHR\0".as_ptr())) };
    self.glDeleteBuffers = unsafe { transmute(loader("glDeleteBuffers\0".as_ptr())) };
    self.glDeleteFramebuffers = unsafe { transmute(loader("glDeleteFramebuffers\0".as_ptr())) };
    self.glDeleteProgram = unsafe { transmute(loader("glDeleteProgram\0".as_ptr())) };
    self.glDeleteProgramPipelines =
      unsafe { transmute(loader("glDeleteProgramPipelines\0".as_ptr())) };
    self.glDeleteQueries = unsafe { transmute(loader("glDeleteQueries\0".as_ptr())) };
    self.glDeleteRenderbuffers = unsafe { transmute(loader("glDeleteRenderbuffers\0".as_ptr())) };
    self.glDeleteSamplers = unsafe { transmute(loader("glDeleteSamplers\0".as_ptr())) };
    self.glDeleteShader = unsafe { transmute(loader("glDeleteShader\0".as_ptr())) };
    self.glDeleteSync = unsafe { transmute(loader("glDeleteSync\0".as_ptr())) };
    self.glDeleteTextures = unsafe { transmute(loader("glDeleteTextures\0".as_ptr())) };
    self.glDeleteTransformFeedbacks =
      unsafe { transmute(loader("glDeleteTransformFeedbacks\0".as_ptr())) };
    self.glDeleteVertexArrays = unsafe { transmute(loader("glDeleteVertexArrays\0".as_ptr())) };
    self.glDepthFunc = unsafe { transmute(loader("glDepthFunc\0".as_ptr())) };
    self.glDepthMask = unsafe { transmute(loader("glDepthMask\0".as_ptr())) };
    self.glDepthRange = unsafe { transmute(loader("glDepthRange\0".as_ptr())) };
    self.glDepthRangeArrayv = unsafe { transmute(loader("glDepthRangeArrayv\0".as_ptr())) };
    self.glDepthRangeIndexed = unsafe { transmute(loader("glDepthRangeIndexed\0".as_ptr())) };
    self.glDepthRangef = unsafe { transmute(loader("glDepthRangef\0".as_ptr())) };
    self.glDetachShader = unsafe { transmute(loader("glDetachShader\0".as_ptr())) };
    self.glDisable = unsafe { transmute(loader("glDisable\0".as_ptr())) };
    self.glDisableVertexArrayAttrib =
      unsafe { transmute(loader("glDisableVertexArrayAttrib\0".as_ptr())) };
    self.glDisableVertexAttribArray =
      unsafe { transmute(loader("glDisableVertexAttribArray\0".as_ptr())) };
    self.glDisablei = unsafe { transmute(loader("glDisablei\0".as_ptr())) };
    self.glDispatchCompute = unsafe { transmute(loader("glDispatchCompute\0".as_ptr())) };
    self.glDispatchComputeIndirect =
      unsafe { transmute(loader("glDispatchComputeIndirect\0".as_ptr())) };
    self.glDrawArrays = unsafe { transmute(loader("glDrawArrays\0".as_ptr())) };
    self.glDrawArraysIndirect = unsafe { transmute(loader("glDrawArraysIndirect\0".as_ptr())) };
    self.glDrawArraysInstanced = unsafe { transmute(loader("glDrawArraysInstanced\0".as_ptr())) };
    self.glDrawArraysInstancedBaseInstance =
      unsafe { transmute(loader("glDrawArraysInstancedBaseInstance\0".as_ptr())) };
    self.glDrawBuffer = unsafe { transmute(loader("glDrawBuffer\0".as_ptr())) };
    self.glDrawBuffers = unsafe { transmute(loader("glDrawBuffers\0".as_ptr())) };
    self.glDrawElements = unsafe { transmute(loader("glDrawElements\0".as_ptr())) };
    self.glDrawElementsBaseVertex =
      unsafe { transmute(loader("glDrawElementsBaseVertex\0".as_ptr())) };
    self.glDrawElementsIndirect = unsafe { transmute(loader("glDrawElementsIndirect\0".as_ptr())) };
    self.glDrawElementsInstanced =
      unsafe { transmute(loader("glDrawElementsInstanced\0".as_ptr())) };
    self.glDrawElementsInstancedBaseInstance =
      unsafe { transmute(loader("glDrawElementsInstancedBaseInstance\0".as_ptr())) };
    self.glDrawElementsInstancedBaseVertex =
      unsafe { transmute(loader("glDrawElementsInstancedBaseVertex\0".as_ptr())) };
    self.glDrawElementsInstancedBaseVertexBaseInstance =
      unsafe { transmute(loader("glDrawElementsInstancedBaseVertexBaseInstance\0".as_ptr())) };
    self.glDrawRangeElements = unsafe { transmute(loader("glDrawRangeElements\0".as_ptr())) };
    self.glDrawRangeElementsBaseVertex =
      unsafe { transmute(loader("glDrawRangeElementsBaseVertex\0".as_ptr())) };
    self.glDrawTransformFeedback =
      unsafe { transmute(loader("glDrawTransformFeedback\0".as_ptr())) };
    self.glDrawTransformFeedbackInstanced =
      unsafe { transmute(loader("glDrawTransformFeedbackInstanced\0".as_ptr())) };
    self.glDrawTransformFeedbackStream =
      unsafe { transmute(loader("glDrawTransformFeedbackStream\0".as_ptr())) };
    self.glDrawTransformFeedbackStreamInstanced =
      unsafe { transmute(loader("glDrawTransformFeedbackStreamInstanced\0".as_ptr())) };
    self.glEnable = unsafe { transmute(loader("glEnable\0".as_ptr())) };
    self.glEnableVertexArrayAttrib =
      unsafe { transmute(loader("glEnableVertexArrayAttrib\0".as_ptr())) };
    self.glEnableVertexAttribArray =
      unsafe { transmute(loader("glEnableVertexAttribArray\0".as_ptr())) };
    self.glEnablei = unsafe { transmute(loader("glEnablei\0".as_ptr())) };
    self.glEndConditionalRender = unsafe { transmute(loader("glEndConditionalRender\0".as_ptr())) };
    self.glEndQuery = unsafe { transmute(loader("glEndQuery\0".as_ptr())) };
    self.glEndQueryIndexed = unsafe { transmute(loader("glEndQueryIndexed\0".as_ptr())) };
    self.glEndTransformFeedback = unsafe { transmute(loader("glEndTransformFeedback\0".as_ptr())) };
    self.glFenceSync = unsafe { transmute(loader("glFenceSync\0".as_ptr())) };
    self.glFinish = unsafe { transmute(loader("glFinish\0".as_ptr())) };
    self.glFlush = unsafe { transmute(loader("glFlush\0".as_ptr())) };
    self.glFlushMappedBufferRange =
      unsafe { transmute(loader("glFlushMappedBufferRange\0".as_ptr())) };
    self.glFlushMappedNamedBufferRange =
      unsafe { transmute(loader("glFlushMappedNamedBufferRange\0".as_ptr())) };
    self.glFramebufferParameteri =
      unsafe { transmute(loader("glFramebufferParameteri\0".as_ptr())) };
    self.glFramebufferRenderbuffer =
      unsafe { transmute(loader("glFramebufferRenderbuffer\0".as_ptr())) };
    self.glFramebufferTexture = unsafe { transmute(loader("glFramebufferTexture\0".as_ptr())) };
    self.glFramebufferTexture1D = unsafe { transmute(loader("glFramebufferTexture1D\0".as_ptr())) };
    self.glFramebufferTexture2D = unsafe { transmute(loader("glFramebufferTexture2D\0".as_ptr())) };
    self.glFramebufferTexture3D = unsafe { transmute(loader("glFramebufferTexture3D\0".as_ptr())) };
    self.glFramebufferTextureLayer =
      unsafe { transmute(loader("glFramebufferTextureLayer\0".as_ptr())) };
    self.glFrontFace = unsafe { transmute(loader("glFrontFace\0".as_ptr())) };
    self.glGenBuffers = unsafe { transmute(loader("glGenBuffers\0".as_ptr())) };
    self.glGenFramebuffers = unsafe { transmute(loader("glGenFramebuffers\0".as_ptr())) };
    self.glGenProgramPipelines = unsafe { transmute(loader("glGenProgramPipelines\0".as_ptr())) };
    self.glGenQueries = unsafe { transmute(loader("glGenQueries\0".as_ptr())) };
    self.glGenRenderbuffers = unsafe { transmute(loader("glGenRenderbuffers\0".as_ptr())) };
    self.glGenSamplers = unsafe { transmute(loader("glGenSamplers\0".as_ptr())) };
    self.glGenTextures = unsafe { transmute(loader("glGenTextures\0".as_ptr())) };
    self.glGenTransformFeedbacks =
      unsafe { transmute(loader("glGenTransformFeedbacks\0".as_ptr())) };
    self.glGenVertexArrays = unsafe { transmute(loader("glGenVertexArrays\0".as_ptr())) };
    self.glGenerateMipmap = unsafe { transmute(loader("glGenerateMipmap\0".as_ptr())) };
    self.glGenerateTextureMipmap =
      unsafe { transmute(loader("glGenerateTextureMipmap\0".as_ptr())) };
    self.glGetActiveAtomicCounterBufferiv =
      unsafe { transmute(loader("glGetActiveAtomicCounterBufferiv\0".as_ptr())) };
    self.glGetActiveAttrib = unsafe { transmute(loader("glGetActiveAttrib\0".as_ptr())) };
    self.glGetActiveSubroutineName =
      unsafe { transmute(loader("glGetActiveSubroutineName\0".as_ptr())) };
    self.glGetActiveSubroutineUniformName =
      unsafe { transmute(loader("glGetActiveSubroutineUniformName\0".as_ptr())) };
    self.glGetActiveSubroutineUniformiv =
      unsafe { transmute(loader("glGetActiveSubroutineUniformiv\0".as_ptr())) };
    self.glGetActiveUniform = unsafe { transmute(loader("glGetActiveUniform\0".as_ptr())) };
    self.glGetActiveUniformBlockName =
      unsafe { transmute(loader("glGetActiveUniformBlockName\0".as_ptr())) };
    self.glGetActiveUniformBlockiv =
      unsafe { transmute(loader("glGetActiveUniformBlockiv\0".as_ptr())) };
    self.glGetActiveUniformName = unsafe { transmute(loader("glGetActiveUniformName\0".as_ptr())) };
    self.glGetActiveUniformsiv = unsafe { transmute(loader("glGetActiveUniformsiv\0".as_ptr())) };
    self.glGetAttachedShaders = unsafe { transmute(loader("glGetAttachedShaders\0".as_ptr())) };
    self.glGetAttribLocation = unsafe { transmute(loader("glGetAttribLocation\0".as_ptr())) };
    self.glGetBooleani_v = unsafe { transmute(loader("glGetBooleani_v\0".as_ptr())) };
    self.glGetBooleanv = unsafe { transmute(loader("glGetBooleanv\0".as_ptr())) };
    self.glGetBufferParameteri64v =
      unsafe { transmute(loader("glGetBufferParameteri64v\0".as_ptr())) };
    self.glGetBufferParameteriv = unsafe { transmute(loader("glGetBufferParameteriv\0".as_ptr())) };
    self.glGetBufferPointerv = unsafe { transmute(loader("glGetBufferPointerv\0".as_ptr())) };
    self.glGetBufferSubData = unsafe { transmute(loader("glGetBufferSubData\0".as_ptr())) };
    self.glGetCompressedTexImage =
      unsafe { transmute(loader("glGetCompressedTexImage\0".as_ptr())) };
    self.glGetCompressedTextureImage =
      unsafe { transmute(loader("glGetCompressedTextureImage\0".as_ptr())) };
    self.glGetCompressedTextureSubImage =
      unsafe { transmute(loader("glGetCompressedTextureSubImage\0".as_ptr())) };
    self.glGetDebugMessageLog = unsafe { transmute(loader("glGetDebugMessageLog\0".as_ptr())) };
    self.glGetDebugMessageLogKHR =
      unsafe { transmute(loader("glGetDebugMessageLogKHR\0".as_ptr())) };
    self.glGetDoublei_v = unsafe { transmute(loader("glGetDoublei_v\0".as_ptr())) };
    self.glGetDoublev = unsafe { transmute(loader("glGetDoublev\0".as_ptr())) };
    self.glGetError = unsafe { transmute(loader("glGetError\0".as_ptr())) };
    self.glGetFloati_v = unsafe { transmute(loader("glGetFloati_v\0".as_ptr())) };
    self.glGetFloatv = unsafe { transmute(loader("glGetFloatv\0".as_ptr())) };
    self.glGetFragDataIndex = unsafe { transmute(loader("glGetFragDataIndex\0".as_ptr())) };
    self.glGetFragDataLocation = unsafe { transmute(loader("glGetFragDataLocation\0".as_ptr())) };
    self.glGetFramebufferAttachmentParameteriv =
      unsafe { transmute(loader("glGetFramebufferAttachmentParameteriv\0".as_ptr())) };
    self.glGetFramebufferParameteriv =
      unsafe { transmute(loader("glGetFramebufferParameteriv\0".as_ptr())) };
    self.glGetGraphicsResetStatus =
      unsafe { transmute(loader("glGetGraphicsResetStatus\0".as_ptr())) };
    self.glGetInteger64i_v = unsafe { transmute(loader("glGetInteger64i_v\0".as_ptr())) };
    self.glGetInteger64v = unsafe { transmute(loader("glGetInteger64v\0".as_ptr())) };
    self.glGetIntegeri_v = unsafe { transmute(loader("glGetIntegeri_v\0".as_ptr())) };
    self.glGetIntegerv = unsafe { transmute(loader("glGetIntegerv\0".as_ptr())) };
    self.glGetInternalformati64v =
      unsafe { transmute(loader("glGetInternalformati64v\0".as_ptr())) };
    self.glGetInternalformativ = unsafe { transmute(loader("glGetInternalformativ\0".as_ptr())) };
    self.glGetMultisamplefv = unsafe { transmute(loader("glGetMultisamplefv\0".as_ptr())) };
    self.glGetNamedBufferParameteri64v =
      unsafe { transmute(loader("glGetNamedBufferParameteri64v\0".as_ptr())) };
    self.glGetNamedBufferParameteriv =
      unsafe { transmute(loader("glGetNamedBufferParameteriv\0".as_ptr())) };
    self.glGetNamedBufferPointerv =
      unsafe { transmute(loader("glGetNamedBufferPointerv\0".as_ptr())) };
    self.glGetNamedBufferSubData =
      unsafe { transmute(loader("glGetNamedBufferSubData\0".as_ptr())) };
    self.glGetNamedFramebufferAttachmentParameteriv =
      unsafe { transmute(loader("glGetNamedFramebufferAttachmentParameteriv\0".as_ptr())) };
    self.glGetNamedFramebufferParameteriv =
      unsafe { transmute(loader("glGetNamedFramebufferParameteriv\0".as_ptr())) };
    self.glGetNamedRenderbufferParameteriv =
      unsafe { transmute(loader("glGetNamedRenderbufferParameteriv\0".as_ptr())) };
    self.glGetObjectLabel = unsafe { transmute(loader("glGetObjectLabel\0".as_ptr())) };
    self.glGetObjectLabelKHR = unsafe { transmute(loader("glGetObjectLabelKHR\0".as_ptr())) };
    self.glGetObjectPtrLabel = unsafe { transmute(loader("glGetObjectPtrLabel\0".as_ptr())) };
    self.glGetObjectPtrLabelKHR = unsafe { transmute(loader("glGetObjectPtrLabelKHR\0".as_ptr())) };
    self.glGetPointerv = unsafe { transmute(loader("glGetPointerv\0".as_ptr())) };
    self.glGetPointervKHR = unsafe { transmute(loader("glGetPointervKHR\0".as_ptr())) };
    self.glGetProgramBinary = unsafe { transmute(loader("glGetProgramBinary\0".as_ptr())) };
    self.glGetProgramInfoLog = unsafe { transmute(loader("glGetProgramInfoLog\0".as_ptr())) };
    self.glGetProgramInterfaceiv =
      unsafe { transmute(loader("glGetProgramInterfaceiv\0".as_ptr())) };
    self.glGetProgramPipelineInfoLog =
      unsafe { transmute(loader("glGetProgramPipelineInfoLog\0".as_ptr())) };
    self.glGetProgramPipelineiv = unsafe { transmute(loader("glGetProgramPipelineiv\0".as_ptr())) };
    self.glGetProgramResourceIndex =
      unsafe { transmute(loader("glGetProgramResourceIndex\0".as_ptr())) };
    self.glGetProgramResourceLocation =
      unsafe { transmute(loader("glGetProgramResourceLocation\0".as_ptr())) };
    self.glGetProgramResourceLocationIndex =
      unsafe { transmute(loader("glGetProgramResourceLocationIndex\0".as_ptr())) };
    self.glGetProgramResourceName =
      unsafe { transmute(loader("glGetProgramResourceName\0".as_ptr())) };
    self.glGetProgramResourceiv = unsafe { transmute(loader("glGetProgramResourceiv\0".as_ptr())) };
    self.glGetProgramStageiv = unsafe { transmute(loader("glGetProgramStageiv\0".as_ptr())) };
    self.glGetProgramiv = unsafe { transmute(loader("glGetProgramiv\0".as_ptr())) };
    self.glGetQueryBufferObjecti64v =
      unsafe { transmute(loader("glGetQueryBufferObjecti64v\0".as_ptr())) };
    self.glGetQueryBufferObjectiv =
      unsafe { transmute(loader("glGetQueryBufferObjectiv\0".as_ptr())) };
    self.glGetQueryBufferObjectui64v =
      unsafe { transmute(loader("glGetQueryBufferObjectui64v\0".as_ptr())) };
    self.glGetQueryBufferObjectuiv =
      unsafe { transmute(loader("glGetQueryBufferObjectuiv\0".as_ptr())) };
    self.glGetQueryIndexediv = unsafe { transmute(loader("glGetQueryIndexediv\0".as_ptr())) };
    self.glGetQueryObjecti64v = unsafe { transmute(loader("glGetQueryObjecti64v\0".as_ptr())) };
    self.glGetQueryObjectiv = unsafe { transmute(loader("glGetQueryObjectiv\0".as_ptr())) };
    self.glGetQueryObjectui64v = unsafe { transmute(loader("glGetQueryObjectui64v\0".as_ptr())) };
    self.glGetQueryObjectuiv = unsafe { transmute(loader("glGetQueryObjectuiv\0".as_ptr())) };
    self.glGetQueryiv = unsafe { transmute(loader("glGetQueryiv\0".as_ptr())) };
    self.glGetRenderbufferParameteriv =
      unsafe { transmute(loader("glGetRenderbufferParameteriv\0".as_ptr())) };
    self.glGetSamplerParameterIiv =
      unsafe { transmute(loader("glGetSamplerParameterIiv\0".as_ptr())) };
    self.glGetSamplerParameterIuiv =
      unsafe { transmute(loader("glGetSamplerParameterIuiv\0".as_ptr())) };
    self.glGetSamplerParameterfv =
      unsafe { transmute(loader("glGetSamplerParameterfv\0".as_ptr())) };
    self.glGetSamplerParameteriv =
      unsafe { transmute(loader("glGetSamplerParameteriv\0".as_ptr())) };
    self.glGetShaderInfoLog = unsafe { transmute(loader("glGetShaderInfoLog\0".as_ptr())) };
    self.glGetShaderPrecisionFormat =
      unsafe { transmute(loader("glGetShaderPrecisionFormat\0".as_ptr())) };
    self.glGetShaderSource = unsafe { transmute(loader("glGetShaderSource\0".as_ptr())) };
    self.glGetShaderiv = unsafe { transmute(loader("glGetShaderiv\0".as_ptr())) };
    self.glGetString = unsafe { transmute(loader("glGetString\0".as_ptr())) };
    self.glGetStringi = unsafe { transmute(loader("glGetStringi\0".as_ptr())) };
    self.glGetSubroutineIndex = unsafe { transmute(loader("glGetSubroutineIndex\0".as_ptr())) };
    self.glGetSubroutineUniformLocation =
      unsafe { transmute(loader("glGetSubroutineUniformLocation\0".as_ptr())) };
    self.glGetSynciv = unsafe { transmute(loader("glGetSynciv\0".as_ptr())) };
    self.glGetTexImage = unsafe { transmute(loader("glGetTexImage\0".as_ptr())) };
    self.glGetTexLevelParameterfv =
      unsafe { transmute(loader("glGetTexLevelParameterfv\0".as_ptr())) };
    self.glGetTexLevelParameteriv =
      unsafe { transmute(loader("glGetTexLevelParameteriv\0".as_ptr())) };
    self.glGetTexParameterIiv = unsafe { transmute(loader("glGetTexParameterIiv\0".as_ptr())) };
    self.glGetTexParameterIuiv = unsafe { transmute(loader("glGetTexParameterIuiv\0".as_ptr())) };
    self.glGetTexParameterfv = unsafe { transmute(loader("glGetTexParameterfv\0".as_ptr())) };
    self.glGetTexParameteriv = unsafe { transmute(loader("glGetTexParameteriv\0".as_ptr())) };
    self.glGetTextureImage = unsafe { transmute(loader("glGetTextureImage\0".as_ptr())) };
    self.glGetTextureLevelParameterfv =
      unsafe { transmute(loader("glGetTextureLevelParameterfv\0".as_ptr())) };
    self.glGetTextureLevelParameteriv =
      unsafe { transmute(loader("glGetTextureLevelParameteriv\0".as_ptr())) };
    self.glGetTextureParameterIiv =
      unsafe { transmute(loader("glGetTextureParameterIiv\0".as_ptr())) };
    self.glGetTextureParameterIuiv =
      unsafe { transmute(loader("glGetTextureParameterIuiv\0".as_ptr())) };
    self.glGetTextureParameterfv =
      unsafe { transmute(loader("glGetTextureParameterfv\0".as_ptr())) };
    self.glGetTextureParameteriv =
      unsafe { transmute(loader("glGetTextureParameteriv\0".as_ptr())) };
    self.glGetTextureSubImage = unsafe { transmute(loader("glGetTextureSubImage\0".as_ptr())) };
    self.glGetTransformFeedbackVarying =
      unsafe { transmute(loader("glGetTransformFeedbackVarying\0".as_ptr())) };
    self.glGetTransformFeedbacki64_v =
      unsafe { transmute(loader("glGetTransformFeedbacki64_v\0".as_ptr())) };
    self.glGetTransformFeedbacki_v =
      unsafe { transmute(loader("glGetTransformFeedbacki_v\0".as_ptr())) };
    self.glGetTransformFeedbackiv =
      unsafe { transmute(loader("glGetTransformFeedbackiv\0".as_ptr())) };
    self.glGetUniformBlockIndex = unsafe { transmute(loader("glGetUniformBlockIndex\0".as_ptr())) };
    self.glGetUniformIndices = unsafe { transmute(loader("glGetUniformIndices\0".as_ptr())) };
    self.glGetUniformLocation = unsafe { transmute(loader("glGetUniformLocation\0".as_ptr())) };
    self.glGetUniformSubroutineuiv =
      unsafe { transmute(loader("glGetUniformSubroutineuiv\0".as_ptr())) };
    self.glGetUniformdv = unsafe { transmute(loader("glGetUniformdv\0".as_ptr())) };
    self.glGetUniformfv = unsafe { transmute(loader("glGetUniformfv\0".as_ptr())) };
    self.glGetUniformiv = unsafe { transmute(loader("glGetUniformiv\0".as_ptr())) };
    self.glGetUniformuiv = unsafe { transmute(loader("glGetUniformuiv\0".as_ptr())) };
    self.glGetVertexArrayIndexed64iv =
      unsafe { transmute(loader("glGetVertexArrayIndexed64iv\0".as_ptr())) };
    self.glGetVertexArrayIndexediv =
      unsafe { transmute(loader("glGetVertexArrayIndexediv\0".as_ptr())) };
    self.glGetVertexArrayiv = unsafe { transmute(loader("glGetVertexArrayiv\0".as_ptr())) };
    self.glGetVertexAttribIiv = unsafe { transmute(loader("glGetVertexAttribIiv\0".as_ptr())) };
    self.glGetVertexAttribIuiv = unsafe { transmute(loader("glGetVertexAttribIuiv\0".as_ptr())) };
    self.glGetVertexAttribLdv = unsafe { transmute(loader("glGetVertexAttribLdv\0".as_ptr())) };
    self.glGetVertexAttribPointerv =
      unsafe { transmute(loader("glGetVertexAttribPointerv\0".as_ptr())) };
    self.glGetVertexAttribdv = unsafe { transmute(loader("glGetVertexAttribdv\0".as_ptr())) };
    self.glGetVertexAttribfv = unsafe { transmute(loader("glGetVertexAttribfv\0".as_ptr())) };
    self.glGetVertexAttribiv = unsafe { transmute(loader("glGetVertexAttribiv\0".as_ptr())) };
    self.glGetnColorTable = unsafe { transmute(loader("glGetnColorTable\0".as_ptr())) };
    self.glGetnCompressedTexImage =
      unsafe { transmute(loader("glGetnCompressedTexImage\0".as_ptr())) };
    self.glGetnConvolutionFilter =
      unsafe { transmute(loader("glGetnConvolutionFilter\0".as_ptr())) };
    self.glGetnHistogram = unsafe { transmute(loader("glGetnHistogram\0".as_ptr())) };
    self.glGetnMapdv = unsafe { transmute(loader("glGetnMapdv\0".as_ptr())) };
    self.glGetnMapfv = unsafe { transmute(loader("glGetnMapfv\0".as_ptr())) };
    self.glGetnMapiv = unsafe { transmute(loader("glGetnMapiv\0".as_ptr())) };
    self.glGetnMinmax = unsafe { transmute(loader("glGetnMinmax\0".as_ptr())) };
    self.glGetnPixelMapfv = unsafe { transmute(loader("glGetnPixelMapfv\0".as_ptr())) };
    self.glGetnPixelMapuiv = unsafe { transmute(loader("glGetnPixelMapuiv\0".as_ptr())) };
    self.glGetnPixelMapusv = unsafe { transmute(loader("glGetnPixelMapusv\0".as_ptr())) };
    self.glGetnPolygonStipple = unsafe { transmute(loader("glGetnPolygonStipple\0".as_ptr())) };
    self.glGetnSeparableFilter = unsafe { transmute(loader("glGetnSeparableFilter\0".as_ptr())) };
    self.glGetnTexImage = unsafe { transmute(loader("glGetnTexImage\0".as_ptr())) };
    self.glGetnUniformdv = unsafe { transmute(loader("glGetnUniformdv\0".as_ptr())) };
    self.glGetnUniformfv = unsafe { transmute(loader("glGetnUniformfv\0".as_ptr())) };
    self.glGetnUniformiv = unsafe { transmute(loader("glGetnUniformiv\0".as_ptr())) };
    self.glGetnUniformuiv = unsafe { transmute(loader("glGetnUniformuiv\0".as_ptr())) };
    self.glHint = unsafe { transmute(loader("glHint\0".as_ptr())) };
    self.glInvalidateBufferData = unsafe { transmute(loader("glInvalidateBufferData\0".as_ptr())) };
    self.glInvalidateBufferSubData =
      unsafe { transmute(loader("glInvalidateBufferSubData\0".as_ptr())) };
    self.glInvalidateFramebuffer =
      unsafe { transmute(loader("glInvalidateFramebuffer\0".as_ptr())) };
    self.glInvalidateNamedFramebufferData =
      unsafe { transmute(loader("glInvalidateNamedFramebufferData\0".as_ptr())) };
    self.glInvalidateNamedFramebufferSubData =
      unsafe { transmute(loader("glInvalidateNamedFramebufferSubData\0".as_ptr())) };
    self.glInvalidateSubFramebuffer =
      unsafe { transmute(loader("glInvalidateSubFramebuffer\0".as_ptr())) };
    self.glInvalidateTexImage = unsafe { transmute(loader("glInvalidateTexImage\0".as_ptr())) };
    self.glInvalidateTexSubImage =
      unsafe { transmute(loader("glInvalidateTexSubImage\0".as_ptr())) };
    self.glIsBuffer = unsafe { transmute(loader("glIsBuffer\0".as_ptr())) };
    self.glIsEnabled = unsafe { transmute(loader("glIsEnabled\0".as_ptr())) };
    self.glIsEnabledi = unsafe { transmute(loader("glIsEnabledi\0".as_ptr())) };
    self.glIsFramebuffer = unsafe { transmute(loader("glIsFramebuffer\0".as_ptr())) };
    self.glIsProgram = unsafe { transmute(loader("glIsProgram\0".as_ptr())) };
    self.glIsProgramPipeline = unsafe { transmute(loader("glIsProgramPipeline\0".as_ptr())) };
    self.glIsQuery = unsafe { transmute(loader("glIsQuery\0".as_ptr())) };
    self.glIsRenderbuffer = unsafe { transmute(loader("glIsRenderbuffer\0".as_ptr())) };
    self.glIsSampler = unsafe { transmute(loader("glIsSampler\0".as_ptr())) };
    self.glIsShader = unsafe { transmute(loader("glIsShader\0".as_ptr())) };
    self.glIsSync = unsafe { transmute(loader("glIsSync\0".as_ptr())) };
    self.glIsTexture = unsafe { transmute(loader("glIsTexture\0".as_ptr())) };
    self.glIsTransformFeedback = unsafe { transmute(loader("glIsTransformFeedback\0".as_ptr())) };
    self.glIsVertexArray = unsafe { transmute(loader("glIsVertexArray\0".as_ptr())) };
    self.glLineWidth = unsafe { transmute(loader("glLineWidth\0".as_ptr())) };
    self.glLinkProgram = unsafe { transmute(loader("glLinkProgram\0".as_ptr())) };
    self.glLogicOp = unsafe { transmute(loader("glLogicOp\0".as_ptr())) };
    self.glMapBuffer = unsafe { transmute(loader("glMapBuffer\0".as_ptr())) };
    self.glMapBufferRange = unsafe { transmute(loader("glMapBufferRange\0".as_ptr())) };
    self.glMapNamedBuffer = unsafe { transmute(loader("glMapNamedBuffer\0".as_ptr())) };
    self.glMapNamedBufferRange = unsafe { transmute(loader("glMapNamedBufferRange\0".as_ptr())) };
    self.glMemoryBarrier = unsafe { transmute(loader("glMemoryBarrier\0".as_ptr())) };
    self.glMemoryBarrierByRegion =
      unsafe { transmute(loader("glMemoryBarrierByRegion\0".as_ptr())) };
    self.glMinSampleShading = unsafe { transmute(loader("glMinSampleShading\0".as_ptr())) };
    self.glMultiDrawArrays = unsafe { transmute(loader("glMultiDrawArrays\0".as_ptr())) };
    self.glMultiDrawArraysIndirect =
      unsafe { transmute(loader("glMultiDrawArraysIndirect\0".as_ptr())) };
    self.glMultiDrawArraysIndirectCount =
      unsafe { transmute(loader("glMultiDrawArraysIndirectCount\0".as_ptr())) };
    self.glMultiDrawElements = unsafe { transmute(loader("glMultiDrawElements\0".as_ptr())) };
    self.glMultiDrawElementsBaseVertex =
      unsafe { transmute(loader("glMultiDrawElementsBaseVertex\0".as_ptr())) };
    self.glMultiDrawElementsIndirect =
      unsafe { transmute(loader("glMultiDrawElementsIndirect\0".as_ptr())) };
    self.glMultiDrawElementsIndirectCount =
      unsafe { transmute(loader("glMultiDrawElementsIndirectCount\0".as_ptr())) };
    self.glMultiTexCoordP1ui = unsafe { transmute(loader("glMultiTexCoordP1ui\0".as_ptr())) };
    self.glMultiTexCoordP1uiv = unsafe { transmute(loader("glMultiTexCoordP1uiv\0".as_ptr())) };
    self.glMultiTexCoordP2ui = unsafe { transmute(loader("glMultiTexCoordP2ui\0".as_ptr())) };
    self.glMultiTexCoordP2uiv = unsafe { transmute(loader("glMultiTexCoordP2uiv\0".as_ptr())) };
    self.glMultiTexCoordP3ui = unsafe { transmute(loader("glMultiTexCoordP3ui\0".as_ptr())) };
    self.glMultiTexCoordP3uiv = unsafe { transmute(loader("glMultiTexCoordP3uiv\0".as_ptr())) };
    self.glMultiTexCoordP4ui = unsafe { transmute(loader("glMultiTexCoordP4ui\0".as_ptr())) };
    self.glMultiTexCoordP4uiv = unsafe { transmute(loader("glMultiTexCoordP4uiv\0".as_ptr())) };
    self.glNamedBufferData = unsafe { transmute(loader("glNamedBufferData\0".as_ptr())) };
    self.glNamedBufferStorage = unsafe { transmute(loader("glNamedBufferStorage\0".as_ptr())) };
    self.glNamedBufferSubData = unsafe { transmute(loader("glNamedBufferSubData\0".as_ptr())) };
    self.glNamedFramebufferDrawBuffer =
      unsafe { transmute(loader("glNamedFramebufferDrawBuffer\0".as_ptr())) };
    self.glNamedFramebufferDrawBuffers =
      unsafe { transmute(loader("glNamedFramebufferDrawBuffers\0".as_ptr())) };
    self.glNamedFramebufferParameteri =
      unsafe { transmute(loader("glNamedFramebufferParameteri\0".as_ptr())) };
    self.glNamedFramebufferReadBuffer =
      unsafe { transmute(loader("glNamedFramebufferReadBuffer\0".as_ptr())) };
    self.glNamedFramebufferRenderbuffer =
      unsafe { transmute(loader("glNamedFramebufferRenderbuffer\0".as_ptr())) };
    self.glNamedFramebufferTexture =
      unsafe { transmute(loader("glNamedFramebufferTexture\0".as_ptr())) };
    self.glNamedFramebufferTextureLayer =
      unsafe { transmute(loader("glNamedFramebufferTextureLayer\0".as_ptr())) };
    self.glNamedRenderbufferStorage =
      unsafe { transmute(loader("glNamedRenderbufferStorage\0".as_ptr())) };
    self.glNamedRenderbufferStorageMultisample =
      unsafe { transmute(loader("glNamedRenderbufferStorageMultisample\0".as_ptr())) };
    self.glNormalP3ui = unsafe { transmute(loader("glNormalP3ui\0".as_ptr())) };
    self.glNormalP3uiv = unsafe { transmute(loader("glNormalP3uiv\0".as_ptr())) };
    self.glObjectLabel = unsafe { transmute(loader("glObjectLabel\0".as_ptr())) };
    self.glObjectLabelKHR = unsafe { transmute(loader("glObjectLabelKHR\0".as_ptr())) };
    self.glObjectPtrLabel = unsafe { transmute(loader("glObjectPtrLabel\0".as_ptr())) };
    self.glObjectPtrLabelKHR = unsafe { transmute(loader("glObjectPtrLabelKHR\0".as_ptr())) };
    self.glPatchParameterfv = unsafe { transmute(loader("glPatchParameterfv\0".as_ptr())) };
    self.glPatchParameteri = unsafe { transmute(loader("glPatchParameteri\0".as_ptr())) };
    self.glPauseTransformFeedback =
      unsafe { transmute(loader("glPauseTransformFeedback\0".as_ptr())) };
    self.glPixelStoref = unsafe { transmute(loader("glPixelStoref\0".as_ptr())) };
    self.glPixelStorei = unsafe { transmute(loader("glPixelStorei\0".as_ptr())) };
    self.glPointParameterf = unsafe { transmute(loader("glPointParameterf\0".as_ptr())) };
    self.glPointParameterfv = unsafe { transmute(loader("glPointParameterfv\0".as_ptr())) };
    self.glPointParameteri = unsafe { transmute(loader("glPointParameteri\0".as_ptr())) };
    self.glPointParameteriv = unsafe { transmute(loader("glPointParameteriv\0".as_ptr())) };
    self.glPointSize = unsafe { transmute(loader("glPointSize\0".as_ptr())) };
    self.glPolygonMode = unsafe { transmute(loader("glPolygonMode\0".as_ptr())) };
    self.glPolygonOffset = unsafe { transmute(loader("glPolygonOffset\0".as_ptr())) };
    self.glPolygonOffsetClamp = unsafe { transmute(loader("glPolygonOffsetClamp\0".as_ptr())) };
    self.glPopDebugGroup = unsafe { transmute(loader("glPopDebugGroup\0".as_ptr())) };
    self.glPopDebugGroupKHR = unsafe { transmute(loader("glPopDebugGroupKHR\0".as_ptr())) };
    self.glPrimitiveBoundingBox = unsafe { transmute(loader("glPrimitiveBoundingBox\0".as_ptr())) };
    self.glPrimitiveRestartIndex =
      unsafe { transmute(loader("glPrimitiveRestartIndex\0".as_ptr())) };
    self.glProgramBinary = unsafe { transmute(loader("glProgramBinary\0".as_ptr())) };
    self.glProgramParameteri = unsafe { transmute(loader("glProgramParameteri\0".as_ptr())) };
    self.glProgramUniform1d = unsafe { transmute(loader("glProgramUniform1d\0".as_ptr())) };
    self.glProgramUniform1dv = unsafe { transmute(loader("glProgramUniform1dv\0".as_ptr())) };
    self.glProgramUniform1f = unsafe { transmute(loader("glProgramUniform1f\0".as_ptr())) };
    self.glProgramUniform1fv = unsafe { transmute(loader("glProgramUniform1fv\0".as_ptr())) };
    self.glProgramUniform1i = unsafe { transmute(loader("glProgramUniform1i\0".as_ptr())) };
    self.glProgramUniform1iv = unsafe { transmute(loader("glProgramUniform1iv\0".as_ptr())) };
    self.glProgramUniform1ui = unsafe { transmute(loader("glProgramUniform1ui\0".as_ptr())) };
    self.glProgramUniform1uiv = unsafe { transmute(loader("glProgramUniform1uiv\0".as_ptr())) };
    self.glProgramUniform2d = unsafe { transmute(loader("glProgramUniform2d\0".as_ptr())) };
    self.glProgramUniform2dv = unsafe { transmute(loader("glProgramUniform2dv\0".as_ptr())) };
    self.glProgramUniform2f = unsafe { transmute(loader("glProgramUniform2f\0".as_ptr())) };
    self.glProgramUniform2fv = unsafe { transmute(loader("glProgramUniform2fv\0".as_ptr())) };
    self.glProgramUniform2i = unsafe { transmute(loader("glProgramUniform2i\0".as_ptr())) };
    self.glProgramUniform2iv = unsafe { transmute(loader("glProgramUniform2iv\0".as_ptr())) };
    self.glProgramUniform2ui = unsafe { transmute(loader("glProgramUniform2ui\0".as_ptr())) };
    self.glProgramUniform2uiv = unsafe { transmute(loader("glProgramUniform2uiv\0".as_ptr())) };
    self.glProgramUniform3d = unsafe { transmute(loader("glProgramUniform3d\0".as_ptr())) };
    self.glProgramUniform3dv = unsafe { transmute(loader("glProgramUniform3dv\0".as_ptr())) };
    self.glProgramUniform3f = unsafe { transmute(loader("glProgramUniform3f\0".as_ptr())) };
    self.glProgramUniform3fv = unsafe { transmute(loader("glProgramUniform3fv\0".as_ptr())) };
    self.glProgramUniform3i = unsafe { transmute(loader("glProgramUniform3i\0".as_ptr())) };
    self.glProgramUniform3iv = unsafe { transmute(loader("glProgramUniform3iv\0".as_ptr())) };
    self.glProgramUniform3ui = unsafe { transmute(loader("glProgramUniform3ui\0".as_ptr())) };
    self.glProgramUniform3uiv = unsafe { transmute(loader("glProgramUniform3uiv\0".as_ptr())) };
    self.glProgramUniform4d = unsafe { transmute(loader("glProgramUniform4d\0".as_ptr())) };
    self.glProgramUniform4dv = unsafe { transmute(loader("glProgramUniform4dv\0".as_ptr())) };
    self.glProgramUniform4f = unsafe { transmute(loader("glProgramUniform4f\0".as_ptr())) };
    self.glProgramUniform4fv = unsafe { transmute(loader("glProgramUniform4fv\0".as_ptr())) };
    self.glProgramUniform4i = unsafe { transmute(loader("glProgramUniform4i\0".as_ptr())) };
    self.glProgramUniform4iv = unsafe { transmute(loader("glProgramUniform4iv\0".as_ptr())) };
    self.glProgramUniform4ui = unsafe { transmute(loader("glProgramUniform4ui\0".as_ptr())) };
    self.glProgramUniform4uiv = unsafe { transmute(loader("glProgramUniform4uiv\0".as_ptr())) };
    self.glProgramUniformMatrix2dv =
      unsafe { transmute(loader("glProgramUniformMatrix2dv\0".as_ptr())) };
    self.glProgramUniformMatrix2fv =
      unsafe { transmute(loader("glProgramUniformMatrix2fv\0".as_ptr())) };
    self.glProgramUniformMatrix2x3dv =
      unsafe { transmute(loader("glProgramUniformMatrix2x3dv\0".as_ptr())) };
    self.glProgramUniformMatrix2x3fv =
      unsafe { transmute(loader("glProgramUniformMatrix2x3fv\0".as_ptr())) };
    self.glProgramUniformMatrix2x4dv =
      unsafe { transmute(loader("glProgramUniformMatrix2x4dv\0".as_ptr())) };
    self.glProgramUniformMatrix2x4fv =
      unsafe { transmute(loader("glProgramUniformMatrix2x4fv\0".as_ptr())) };
    self.glProgramUniformMatrix3dv =
      unsafe { transmute(loader("glProgramUniformMatrix3dv\0".as_ptr())) };
    self.glProgramUniformMatrix3fv =
      unsafe { transmute(loader("glProgramUniformMatrix3fv\0".as_ptr())) };
    self.glProgramUniformMatrix3x2dv =
      unsafe { transmute(loader("glProgramUniformMatrix3x2dv\0".as_ptr())) };
    self.glProgramUniformMatrix3x2fv =
      unsafe { transmute(loader("glProgramUniformMatrix3x2fv\0".as_ptr())) };
    self.glProgramUniformMatrix3x4dv =
      unsafe { transmute(loader("glProgramUniformMatrix3x4dv\0".as_ptr())) };
    self.glProgramUniformMatrix3x4fv =
      unsafe { transmute(loader("glProgramUniformMatrix3x4fv\0".as_ptr())) };
    self.glProgramUniformMatrix4dv =
      unsafe { transmute(loader("glProgramUniformMatrix4dv\0".as_ptr())) };
    self.glProgramUniformMatrix4fv =
      unsafe { transmute(loader("glProgramUniformMatrix4fv\0".as_ptr())) };
    self.glProgramUniformMatrix4x2dv =
      unsafe { transmute(loader("glProgramUniformMatrix4x2dv\0".as_ptr())) };
    self.glProgramUniformMatrix4x2fv =
      unsafe { transmute(loader("glProgramUniformMatrix4x2fv\0".as_ptr())) };
    self.glProgramUniformMatrix4x3dv =
      unsafe { transmute(loader("glProgramUniformMatrix4x3dv\0".as_ptr())) };
    self.glProgramUniformMatrix4x3fv =
      unsafe { transmute(loader("glProgramUniformMatrix4x3fv\0".as_ptr())) };
    self.glProvokingVertex = unsafe { transmute(loader("glProvokingVertex\0".as_ptr())) };
    self.glPushDebugGroup = unsafe { transmute(loader("glPushDebugGroup\0".as_ptr())) };
    self.glPushDebugGroupKHR = unsafe { transmute(loader("glPushDebugGroupKHR\0".as_ptr())) };
    self.glQueryCounter = unsafe { transmute(loader("glQueryCounter\0".as_ptr())) };
    self.glReadBuffer = unsafe { transmute(loader("glReadBuffer\0".as_ptr())) };
    self.glReadPixels = unsafe { transmute(loader("glReadPixels\0".as_ptr())) };
    self.glReadnPixels = unsafe { transmute(loader("glReadnPixels\0".as_ptr())) };
    self.glReleaseShaderCompiler =
      unsafe { transmute(loader("glReleaseShaderCompiler\0".as_ptr())) };
    self.glRenderbufferStorage = unsafe { transmute(loader("glRenderbufferStorage\0".as_ptr())) };
    self.glRenderbufferStorageMultisample =
      unsafe { transmute(loader("glRenderbufferStorageMultisample\0".as_ptr())) };
    self.glResumeTransformFeedback =
      unsafe { transmute(loader("glResumeTransformFeedback\0".as_ptr())) };
    self.glSampleCoverage = unsafe { transmute(loader("glSampleCoverage\0".as_ptr())) };
    self.glSampleMaski = unsafe { transmute(loader("glSampleMaski\0".as_ptr())) };
    self.glSamplerParameterIiv = unsafe { transmute(loader("glSamplerParameterIiv\0".as_ptr())) };
    self.glSamplerParameterIuiv = unsafe { transmute(loader("glSamplerParameterIuiv\0".as_ptr())) };
    self.glSamplerParameterf = unsafe { transmute(loader("glSamplerParameterf\0".as_ptr())) };
    self.glSamplerParameterfv = unsafe { transmute(loader("glSamplerParameterfv\0".as_ptr())) };
    self.glSamplerParameteri = unsafe { transmute(loader("glSamplerParameteri\0".as_ptr())) };
    self.glSamplerParameteriv = unsafe { transmute(loader("glSamplerParameteriv\0".as_ptr())) };
    self.glScissor = unsafe { transmute(loader("glScissor\0".as_ptr())) };
    self.glScissorArrayv = unsafe { transmute(loader("glScissorArrayv\0".as_ptr())) };
    self.glScissorIndexed = unsafe { transmute(loader("glScissorIndexed\0".as_ptr())) };
    self.glScissorIndexedv = unsafe { transmute(loader("glScissorIndexedv\0".as_ptr())) };
    self.glSecondaryColorP3ui = unsafe { transmute(loader("glSecondaryColorP3ui\0".as_ptr())) };
    self.glSecondaryColorP3uiv = unsafe { transmute(loader("glSecondaryColorP3uiv\0".as_ptr())) };
    self.glShaderBinary = unsafe { transmute(loader("glShaderBinary\0".as_ptr())) };
    self.glShaderSource = unsafe { transmute(loader("glShaderSource\0".as_ptr())) };
    self.glShaderStorageBlockBinding =
      unsafe { transmute(loader("glShaderStorageBlockBinding\0".as_ptr())) };
    self.glSpecializeShader = unsafe { transmute(loader("glSpecializeShader\0".as_ptr())) };
    self.glStencilFunc = unsafe { transmute(loader("glStencilFunc\0".as_ptr())) };
    self.glStencilFuncSeparate = unsafe { transmute(loader("glStencilFuncSeparate\0".as_ptr())) };
    self.glStencilMask = unsafe { transmute(loader("glStencilMask\0".as_ptr())) };
    self.glStencilMaskSeparate = unsafe { transmute(loader("glStencilMaskSeparate\0".as_ptr())) };
    self.glStencilOp = unsafe { transmute(loader("glStencilOp\0".as_ptr())) };
    self.glStencilOpSeparate = unsafe { transmute(loader("glStencilOpSeparate\0".as_ptr())) };
    self.glTexBuffer = unsafe { transmute(loader("glTexBuffer\0".as_ptr())) };
    self.glTexBufferRange = unsafe { transmute(loader("glTexBufferRange\0".as_ptr())) };
    self.glTexCoordP1ui = unsafe { transmute(loader("glTexCoordP1ui\0".as_ptr())) };
    self.glTexCoordP1uiv = unsafe { transmute(loader("glTexCoordP1uiv\0".as_ptr())) };
    self.glTexCoordP2ui = unsafe { transmute(loader("glTexCoordP2ui\0".as_ptr())) };
    self.glTexCoordP2uiv = unsafe { transmute(loader("glTexCoordP2uiv\0".as_ptr())) };
    self.glTexCoordP3ui = unsafe { transmute(loader("glTexCoordP3ui\0".as_ptr())) };
    self.glTexCoordP3uiv = unsafe { transmute(loader("glTexCoordP3uiv\0".as_ptr())) };
    self.glTexCoordP4ui = unsafe { transmute(loader("glTexCoordP4ui\0".as_ptr())) };
    self.glTexCoordP4uiv = unsafe { transmute(loader("glTexCoordP4uiv\0".as_ptr())) };
    self.glTexImage1D = unsafe { transmute(loader("glTexImage1D\0".as_ptr())) };
    self.glTexImage2D = unsafe { transmute(loader("glTexImage2D\0".as_ptr())) };
    self.glTexImage2DMultisample =
      unsafe { transmute(loader("glTexImage2DMultisample\0".as_ptr())) };
    self.glTexImage3D = unsafe { transmute(loader("glTexImage3D\0".as_ptr())) };
    self.glTexImage3DMultisample =
      unsafe { transmute(loader("glTexImage3DMultisample\0".as_ptr())) };
    self.glTexParameterIiv = unsafe { transmute(loader("glTexParameterIiv\0".as_ptr())) };
    self.glTexParameterIuiv = unsafe { transmute(loader("glTexParameterIuiv\0".as_ptr())) };
    self.glTexParameterf = unsafe { transmute(loader("glTexParameterf\0".as_ptr())) };
    self.glTexParameterfv = unsafe { transmute(loader("glTexParameterfv\0".as_ptr())) };
    self.glTexParameteri = unsafe { transmute(loader("glTexParameteri\0".as_ptr())) };
    self.glTexParameteriv = unsafe { transmute(loader("glTexParameteriv\0".as_ptr())) };
    self.glTexStorage1D = unsafe { transmute(loader("glTexStorage1D\0".as_ptr())) };
    self.glTexStorage2D = unsafe { transmute(loader("glTexStorage2D\0".as_ptr())) };
    self.glTexStorage2DMultisample =
      unsafe { transmute(loader("glTexStorage2DMultisample\0".as_ptr())) };
    self.glTexStorage3D = unsafe { transmute(loader("glTexStorage3D\0".as_ptr())) };
    self.glTexStorage3DMultisample =
      unsafe { transmute(loader("glTexStorage3DMultisample\0".as_ptr())) };
    self.glTexSubImage1D = unsafe { transmute(loader("glTexSubImage1D\0".as_ptr())) };
    self.glTexSubImage2D = unsafe { transmute(loader("glTexSubImage2D\0".as_ptr())) };
    self.glTexSubImage3D = unsafe { transmute(loader("glTexSubImage3D\0".as_ptr())) };
    self.glTextureBarrier = unsafe { transmute(loader("glTextureBarrier\0".as_ptr())) };
    self.glTextureBuffer = unsafe { transmute(loader("glTextureBuffer\0".as_ptr())) };
    self.glTextureBufferRange = unsafe { transmute(loader("glTextureBufferRange\0".as_ptr())) };
    self.glTextureParameterIiv = unsafe { transmute(loader("glTextureParameterIiv\0".as_ptr())) };
    self.glTextureParameterIuiv = unsafe { transmute(loader("glTextureParameterIuiv\0".as_ptr())) };
    self.glTextureParameterf = unsafe { transmute(loader("glTextureParameterf\0".as_ptr())) };
    self.glTextureParameterfv = unsafe { transmute(loader("glTextureParameterfv\0".as_ptr())) };
    self.glTextureParameteri = unsafe { transmute(loader("glTextureParameteri\0".as_ptr())) };
    self.glTextureParameteriv = unsafe { transmute(loader("glTextureParameteriv\0".as_ptr())) };
    self.glTextureStorage1D = unsafe { transmute(loader("glTextureStorage1D\0".as_ptr())) };
    self.glTextureStorage2D = unsafe { transmute(loader("glTextureStorage2D\0".as_ptr())) };
    self.glTextureStorage2DMultisample =
      unsafe { transmute(loader("glTextureStorage2DMultisample\0".as_ptr())) };
    self.glTextureStorage3D = unsafe { transmute(loader("glTextureStorage3D\0".as_ptr())) };
    self.glTextureStorage3DMultisample =
      unsafe { transmute(loader("glTextureStorage3DMultisample\0".as_ptr())) };
    self.glTextureSubImage1D = unsafe { transmute(loader("glTextureSubImage1D\0".as_ptr())) };
    self.glTextureSubImage2D = unsafe { transmute(loader("glTextureSubImage2D\0".as_ptr())) };
    self.glTextureSubImage3D = unsafe { transmute(loader("glTextureSubImage3D\0".as_ptr())) };
    self.glTextureView = unsafe { transmute(loader("glTextureView\0".as_ptr())) };
    self.glTransformFeedbackBufferBase =
      unsafe { transmute(loader("glTransformFeedbackBufferBase\0".as_ptr())) };
    self.glTransformFeedbackBufferRange =
      unsafe { transmute(loader("glTransformFeedbackBufferRange\0".as_ptr())) };
    self.glTransformFeedbackVaryings =
      unsafe { transmute(loader("glTransformFeedbackVaryings\0".as_ptr())) };
    self.glUniform1d = unsafe { transmute(loader("glUniform1d\0".as_ptr())) };
    self.glUniform1dv = unsafe { transmute(loader("glUniform1dv\0".as_ptr())) };
    self.glUniform1f = unsafe { transmute(loader("glUniform1f\0".as_ptr())) };
    self.glUniform1fv = unsafe { transmute(loader("glUniform1fv\0".as_ptr())) };
    self.glUniform1i = unsafe { transmute(loader("glUniform1i\0".as_ptr())) };
    self.glUniform1iv = unsafe { transmute(loader("glUniform1iv\0".as_ptr())) };
    self.glUniform1ui = unsafe { transmute(loader("glUniform1ui\0".as_ptr())) };
    self.glUniform1uiv = unsafe { transmute(loader("glUniform1uiv\0".as_ptr())) };
    self.glUniform2d = unsafe { transmute(loader("glUniform2d\0".as_ptr())) };
    self.glUniform2dv = unsafe { transmute(loader("glUniform2dv\0".as_ptr())) };
    self.glUniform2f = unsafe { transmute(loader("glUniform2f\0".as_ptr())) };
    self.glUniform2fv = unsafe { transmute(loader("glUniform2fv\0".as_ptr())) };
    self.glUniform2i = unsafe { transmute(loader("glUniform2i\0".as_ptr())) };
    self.glUniform2iv = unsafe { transmute(loader("glUniform2iv\0".as_ptr())) };
    self.glUniform2ui = unsafe { transmute(loader("glUniform2ui\0".as_ptr())) };
    self.glUniform2uiv = unsafe { transmute(loader("glUniform2uiv\0".as_ptr())) };
    self.glUniform3d = unsafe { transmute(loader("glUniform3d\0".as_ptr())) };
    self.glUniform3dv = unsafe { transmute(loader("glUniform3dv\0".as_ptr())) };
    self.glUniform3f = unsafe { transmute(loader("glUniform3f\0".as_ptr())) };
    self.glUniform3fv = unsafe { transmute(loader("glUniform3fv\0".as_ptr())) };
    self.glUniform3i = unsafe { transmute(loader("glUniform3i\0".as_ptr())) };
    self.glUniform3iv = unsafe { transmute(loader("glUniform3iv\0".as_ptr())) };
    self.glUniform3ui = unsafe { transmute(loader("glUniform3ui\0".as_ptr())) };
    self.glUniform3uiv = unsafe { transmute(loader("glUniform3uiv\0".as_ptr())) };
    self.glUniform4d = unsafe { transmute(loader("glUniform4d\0".as_ptr())) };
    self.glUniform4dv = unsafe { transmute(loader("glUniform4dv\0".as_ptr())) };
    self.glUniform4f = unsafe { transmute(loader("glUniform4f\0".as_ptr())) };
    self.glUniform4fv = unsafe { transmute(loader("glUniform4fv\0".as_ptr())) };
    self.glUniform4i = unsafe { transmute(loader("glUniform4i\0".as_ptr())) };
    self.glUniform4iv = unsafe { transmute(loader("glUniform4iv\0".as_ptr())) };
    self.glUniform4ui = unsafe { transmute(loader("glUniform4ui\0".as_ptr())) };
    self.glUniform4uiv = unsafe { transmute(loader("glUniform4uiv\0".as_ptr())) };
    self.glUniformBlockBinding = unsafe { transmute(loader("glUniformBlockBinding\0".as_ptr())) };
    self.glUniformMatrix2dv = unsafe { transmute(loader("glUniformMatrix2dv\0".as_ptr())) };
    self.glUniformMatrix2fv = unsafe { transmute(loader("glUniformMatrix2fv\0".as_ptr())) };
    self.glUniformMatrix2x3dv = unsafe { transmute(loader("glUniformMatrix2x3dv\0".as_ptr())) };
    self.glUniformMatrix2x3fv = unsafe { transmute(loader("glUniformMatrix2x3fv\0".as_ptr())) };
    self.glUniformMatrix2x4dv = unsafe { transmute(loader("glUniformMatrix2x4dv\0".as_ptr())) };
    self.glUniformMatrix2x4fv = unsafe { transmute(loader("glUniformMatrix2x4fv\0".as_ptr())) };
    self.glUniformMatrix3dv = unsafe { transmute(loader("glUniformMatrix3dv\0".as_ptr())) };
    self.glUniformMatrix3fv = unsafe { transmute(loader("glUniformMatrix3fv\0".as_ptr())) };
    self.glUniformMatrix3x2dv = unsafe { transmute(loader("glUniformMatrix3x2dv\0".as_ptr())) };
    self.glUniformMatrix3x2fv = unsafe { transmute(loader("glUniformMatrix3x2fv\0".as_ptr())) };
    self.glUniformMatrix3x4dv = unsafe { transmute(loader("glUniformMatrix3x4dv\0".as_ptr())) };
    self.glUniformMatrix3x4fv = unsafe { transmute(loader("glUniformMatrix3x4fv\0".as_ptr())) };
    self.glUniformMatrix4dv = unsafe { transmute(loader("glUniformMatrix4dv\0".as_ptr())) };
    self.glUniformMatrix4fv = unsafe { transmute(loader("glUniformMatrix4fv\0".as_ptr())) };
    self.glUniformMatrix4x2dv = unsafe { transmute(loader("glUniformMatrix4x2dv\0".as_ptr())) };
    self.glUniformMatrix4x2fv = unsafe { transmute(loader("glUniformMatrix4x2fv\0".as_ptr())) };
    self.glUniformMatrix4x3dv = unsafe { transmute(loader("glUniformMatrix4x3dv\0".as_ptr())) };
    self.glUniformMatrix4x3fv = unsafe { transmute(loader("glUniformMatrix4x3fv\0".as_ptr())) };
    self.glUniformSubroutinesuiv =
      unsafe { transmute(loader("glUniformSubroutinesuiv\0".as_ptr())) };
    self.glUnmapBuffer = unsafe { transmute(loader("glUnmapBuffer\0".as_ptr())) };
    self.glUnmapNamedBuffer = unsafe { transmute(loader("glUnmapNamedBuffer\0".as_ptr())) };
    self.glUseProgram = unsafe { transmute(loader("glUseProgram\0".as_ptr())) };
    self.glUseProgramStages = unsafe { transmute(loader("glUseProgramStages\0".as_ptr())) };
    self.glValidateProgram = unsafe { transmute(loader("glValidateProgram\0".as_ptr())) };
    self.glValidateProgramPipeline =
      unsafe { transmute(loader("glValidateProgramPipeline\0".as_ptr())) };
    self.glVertexArrayAttribBinding =
      unsafe { transmute(loader("glVertexArrayAttribBinding\0".as_ptr())) };
    self.glVertexArrayAttribFormat =
      unsafe { transmute(loader("glVertexArrayAttribFormat\0".as_ptr())) };
    self.glVertexArrayAttribIFormat =
      unsafe { transmute(loader("glVertexArrayAttribIFormat\0".as_ptr())) };
    self.glVertexArrayAttribLFormat =
      unsafe { transmute(loader("glVertexArrayAttribLFormat\0".as_ptr())) };
    self.glVertexArrayBindingDivisor =
      unsafe { transmute(loader("glVertexArrayBindingDivisor\0".as_ptr())) };
    self.glVertexArrayElementBuffer =
      unsafe { transmute(loader("glVertexArrayElementBuffer\0".as_ptr())) };
    self.glVertexArrayVertexBuffer =
      unsafe { transmute(loader("glVertexArrayVertexBuffer\0".as_ptr())) };
    self.glVertexArrayVertexBuffers =
      unsafe { transmute(loader("glVertexArrayVertexBuffers\0".as_ptr())) };
    self.glVertexAttrib1d = unsafe { transmute(loader("glVertexAttrib1d\0".as_ptr())) };
    self.glVertexAttrib1dv = unsafe { transmute(loader("glVertexAttrib1dv\0".as_ptr())) };
    self.glVertexAttrib1f = unsafe { transmute(loader("glVertexAttrib1f\0".as_ptr())) };
    self.glVertexAttrib1fv = unsafe { transmute(loader("glVertexAttrib1fv\0".as_ptr())) };
    self.glVertexAttrib1s = unsafe { transmute(loader("glVertexAttrib1s\0".as_ptr())) };
    self.glVertexAttrib1sv = unsafe { transmute(loader("glVertexAttrib1sv\0".as_ptr())) };
    self.glVertexAttrib2d = unsafe { transmute(loader("glVertexAttrib2d\0".as_ptr())) };
    self.glVertexAttrib2dv = unsafe { transmute(loader("glVertexAttrib2dv\0".as_ptr())) };
    self.glVertexAttrib2f = unsafe { transmute(loader("glVertexAttrib2f\0".as_ptr())) };
    self.glVertexAttrib2fv = unsafe { transmute(loader("glVertexAttrib2fv\0".as_ptr())) };
    self.glVertexAttrib2s = unsafe { transmute(loader("glVertexAttrib2s\0".as_ptr())) };
    self.glVertexAttrib2sv = unsafe { transmute(loader("glVertexAttrib2sv\0".as_ptr())) };
    self.glVertexAttrib3d = unsafe { transmute(loader("glVertexAttrib3d\0".as_ptr())) };
    self.glVertexAttrib3dv = unsafe { transmute(loader("glVertexAttrib3dv\0".as_ptr())) };
    self.glVertexAttrib3f = unsafe { transmute(loader("glVertexAttrib3f\0".as_ptr())) };
    self.glVertexAttrib3fv = unsafe { transmute(loader("glVertexAttrib3fv\0".as_ptr())) };
    self.glVertexAttrib3s = unsafe { transmute(loader("glVertexAttrib3s\0".as_ptr())) };
    self.glVertexAttrib3sv = unsafe { transmute(loader("glVertexAttrib3sv\0".as_ptr())) };
    self.glVertexAttrib4Nbv = unsafe { transmute(loader("glVertexAttrib4Nbv\0".as_ptr())) };
    self.glVertexAttrib4Niv = unsafe { transmute(loader("glVertexAttrib4Niv\0".as_ptr())) };
    self.glVertexAttrib4Nsv = unsafe { transmute(loader("glVertexAttrib4Nsv\0".as_ptr())) };
    self.glVertexAttrib4Nub = unsafe { transmute(loader("glVertexAttrib4Nub\0".as_ptr())) };
    self.glVertexAttrib4Nubv = unsafe { transmute(loader("glVertexAttrib4Nubv\0".as_ptr())) };
    self.glVertexAttrib4Nuiv = unsafe { transmute(loader("glVertexAttrib4Nuiv\0".as_ptr())) };
    self.glVertexAttrib4Nusv = unsafe { transmute(loader("glVertexAttrib4Nusv\0".as_ptr())) };
    self.glVertexAttrib4bv = unsafe { transmute(loader("glVertexAttrib4bv\0".as_ptr())) };
    self.glVertexAttrib4d = unsafe { transmute(loader("glVertexAttrib4d\0".as_ptr())) };
    self.glVertexAttrib4dv = unsafe { transmute(loader("glVertexAttrib4dv\0".as_ptr())) };
    self.glVertexAttrib4f = unsafe { transmute(loader("glVertexAttrib4f\0".as_ptr())) };
    self.glVertexAttrib4fv = unsafe { transmute(loader("glVertexAttrib4fv\0".as_ptr())) };
    self.glVertexAttrib4iv = unsafe { transmute(loader("glVertexAttrib4iv\0".as_ptr())) };
    self.glVertexAttrib4s = unsafe { transmute(loader("glVertexAttrib4s\0".as_ptr())) };
    self.glVertexAttrib4sv = unsafe { transmute(loader("glVertexAttrib4sv\0".as_ptr())) };
    self.glVertexAttrib4ubv = unsafe { transmute(loader("glVertexAttrib4ubv\0".as_ptr())) };
    self.glVertexAttrib4uiv = unsafe { transmute(loader("glVertexAttrib4uiv\0".as_ptr())) };
    self.glVertexAttrib4usv = unsafe { transmute(loader("glVertexAttrib4usv\0".as_ptr())) };
    self.glVertexAttribBinding = unsafe { transmute(loader("glVertexAttribBinding\0".as_ptr())) };
    self.glVertexAttribDivisor = unsafe { transmute(loader("glVertexAttribDivisor\0".as_ptr())) };
    self.glVertexAttribFormat = unsafe { transmute(loader("glVertexAttribFormat\0".as_ptr())) };
    self.glVertexAttribI1i = unsafe { transmute(loader("glVertexAttribI1i\0".as_ptr())) };
    self.glVertexAttribI1iv = unsafe { transmute(loader("glVertexAttribI1iv\0".as_ptr())) };
    self.glVertexAttribI1ui = unsafe { transmute(loader("glVertexAttribI1ui\0".as_ptr())) };
    self.glVertexAttribI1uiv = unsafe { transmute(loader("glVertexAttribI1uiv\0".as_ptr())) };
    self.glVertexAttribI2i = unsafe { transmute(loader("glVertexAttribI2i\0".as_ptr())) };
    self.glVertexAttribI2iv = unsafe { transmute(loader("glVertexAttribI2iv\0".as_ptr())) };
    self.glVertexAttribI2ui = unsafe { transmute(loader("glVertexAttribI2ui\0".as_ptr())) };
    self.glVertexAttribI2uiv = unsafe { transmute(loader("glVertexAttribI2uiv\0".as_ptr())) };
    self.glVertexAttribI3i = unsafe { transmute(loader("glVertexAttribI3i\0".as_ptr())) };
    self.glVertexAttribI3iv = unsafe { transmute(loader("glVertexAttribI3iv\0".as_ptr())) };
    self.glVertexAttribI3ui = unsafe { transmute(loader("glVertexAttribI3ui\0".as_ptr())) };
    self.glVertexAttribI3uiv = unsafe { transmute(loader("glVertexAttribI3uiv\0".as_ptr())) };
    self.glVertexAttribI4bv = unsafe { transmute(loader("glVertexAttribI4bv\0".as_ptr())) };
    self.glVertexAttribI4i = unsafe { transmute(loader("glVertexAttribI4i\0".as_ptr())) };
    self.glVertexAttribI4iv = unsafe { transmute(loader("glVertexAttribI4iv\0".as_ptr())) };
    self.glVertexAttribI4sv = unsafe { transmute(loader("glVertexAttribI4sv\0".as_ptr())) };
    self.glVertexAttribI4ubv = unsafe { transmute(loader("glVertexAttribI4ubv\0".as_ptr())) };
    self.glVertexAttribI4ui = unsafe { transmute(loader("glVertexAttribI4ui\0".as_ptr())) };
    self.glVertexAttribI4uiv = unsafe { transmute(loader("glVertexAttribI4uiv\0".as_ptr())) };
    self.glVertexAttribI4usv = unsafe { transmute(loader("glVertexAttribI4usv\0".as_ptr())) };
    self.glVertexAttribIFormat = unsafe { transmute(loader("glVertexAttribIFormat\0".as_ptr())) };
    self.glVertexAttribIPointer = unsafe { transmute(loader("glVertexAttribIPointer\0".as_ptr())) };
    self.glVertexAttribL1d = unsafe { transmute(loader("glVertexAttribL1d\0".as_ptr())) };
    self.glVertexAttribL1dv = unsafe { transmute(loader("glVertexAttribL1dv\0".as_ptr())) };
    self.glVertexAttribL2d = unsafe { transmute(loader("glVertexAttribL2d\0".as_ptr())) };
    self.glVertexAttribL2dv = unsafe { transmute(loader("glVertexAttribL2dv\0".as_ptr())) };
    self.glVertexAttribL3d = unsafe { transmute(loader("glVertexAttribL3d\0".as_ptr())) };
    self.glVertexAttribL3dv = unsafe { transmute(loader("glVertexAttribL3dv\0".as_ptr())) };
    self.glVertexAttribL4d = unsafe { transmute(loader("glVertexAttribL4d\0".as_ptr())) };
    self.glVertexAttribL4dv = unsafe { transmute(loader("glVertexAttribL4dv\0".as_ptr())) };
    self.glVertexAttribLFormat = unsafe { transmute(loader("glVertexAttribLFormat\0".as_ptr())) };
    self.glVertexAttribLPointer = unsafe { transmute(loader("glVertexAttribLPointer\0".as_ptr())) };
    self.glVertexAttribP1ui = unsafe { transmute(loader("glVertexAttribP1ui\0".as_ptr())) };
    self.glVertexAttribP1uiv = unsafe { transmute(loader("glVertexAttribP1uiv\0".as_ptr())) };
    self.glVertexAttribP2ui = unsafe { transmute(loader("glVertexAttribP2ui\0".as_ptr())) };
    self.glVertexAttribP2uiv = unsafe { transmute(loader("glVertexAttribP2uiv\0".as_ptr())) };
    self.glVertexAttribP3ui = unsafe { transmute(loader("glVertexAttribP3ui\0".as_ptr())) };
    self.glVertexAttribP3uiv = unsafe { transmute(loader("glVertexAttribP3uiv\0".as_ptr())) };
    self.glVertexAttribP4ui = unsafe { transmute(loader("glVertexAttribP4ui\0".as_ptr())) };
    self.glVertexAttribP4uiv = unsafe { transmute(loader("glVertexAttribP4uiv\0".as_ptr())) };
    self.glVertexAttribPointer = unsafe { transmute(loader("glVertexAttribPointer\0".as_ptr())) };
    self.glVertexBindingDivisor = unsafe { transmute(loader("glVertexBindingDivisor\0".as_ptr())) };
    self.glVertexP2ui = unsafe { transmute(loader("glVertexP2ui\0".as_ptr())) };
    self.glVertexP2uiv = unsafe { transmute(loader("glVertexP2uiv\0".as_ptr())) };
    self.glVertexP3ui = unsafe { transmute(loader("glVertexP3ui\0".as_ptr())) };
    self.glVertexP3uiv = unsafe { transmute(loader("glVertexP3uiv\0".as_ptr())) };
    self.glVertexP4ui = unsafe { transmute(loader("glVertexP4ui\0".as_ptr())) };
    self.glVertexP4uiv = unsafe { transmute(loader("glVertexP4uiv\0".as_ptr())) };
    self.glViewport = unsafe { transmute(loader("glViewport\0".as_ptr())) };
    self.glViewportArrayv = unsafe { transmute(loader("glViewportArrayv\0".as_ptr())) };
    self.glViewportIndexedf = unsafe { transmute(loader("glViewportIndexedf\0".as_ptr())) };
    self.glViewportIndexedfv = unsafe { transmute(loader("glViewportIndexedfv\0".as_ptr())) };
    self.glWaitSync = unsafe { transmute(loader("glWaitSync\0".as_ptr())) };
  }
}

/// A `GlFns` with all fields unloaded.
///
/// This is intended to be of use if you need to initialize a `static` value.
pub const BLANK_GL_FNS: GlFns = GlFns {
  glActiveShaderProgram: None,
  glActiveTexture: None,
  glAttachShader: None,
  glBeginConditionalRender: None,
  glBeginQuery: None,
  glBeginQueryIndexed: None,
  glBeginTransformFeedback: None,
  glBindAttribLocation: None,
  glBindBuffer: None,
  glBindBufferBase: None,
  glBindBufferRange: None,
  glBindBuffersBase: None,
  glBindBuffersRange: None,
  glBindFragDataLocation: None,
  glBindFragDataLocationIndexed: None,
  glBindFramebuffer: None,
  glBindImageTexture: None,
  glBindImageTextures: None,
  glBindProgramPipeline: None,
  glBindRenderbuffer: None,
  glBindSampler: None,
  glBindSamplers: None,
  glBindTexture: None,
  glBindTextureUnit: None,
  glBindTextures: None,
  glBindTransformFeedback: None,
  glBindVertexArray: None,
  glBindVertexBuffer: None,
  glBindVertexBuffers: None,
  glBlendBarrier: None,
  glBlendColor: None,
  glBlendEquation: None,
  glBlendEquationSeparate: None,
  glBlendEquationSeparatei: None,
  glBlendEquationi: None,
  glBlendFunc: None,
  glBlendFuncSeparate: None,
  glBlendFuncSeparatei: None,
  glBlendFunci: None,
  glBlitFramebuffer: None,
  glBlitNamedFramebuffer: None,
  glBufferData: None,
  glBufferStorage: None,
  glBufferSubData: None,
  glCheckFramebufferStatus: None,
  glCheckNamedFramebufferStatus: None,
  glClampColor: None,
  glClear: None,
  glClearBufferData: None,
  glClearBufferSubData: None,
  glClearBufferfi: None,
  glClearBufferfv: None,
  glClearBufferiv: None,
  glClearBufferuiv: None,
  glClearColor: None,
  glClearDepth: None,
  glClearDepthf: None,
  glClearNamedBufferData: None,
  glClearNamedBufferSubData: None,
  glClearNamedFramebufferfi: None,
  glClearNamedFramebufferfv: None,
  glClearNamedFramebufferiv: None,
  glClearNamedFramebufferuiv: None,
  glClearStencil: None,
  glClearTexImage: None,
  glClearTexSubImage: None,
  glClientWaitSync: None,
  glClipControl: None,
  glColorMask: None,
  glColorMaski: None,
  glColorP3ui: None,
  glColorP3uiv: None,
  glColorP4ui: None,
  glColorP4uiv: None,
  glCompileShader: None,
  glCompressedTexImage1D: None,
  glCompressedTexImage2D: None,
  glCompressedTexImage3D: None,
  glCompressedTexSubImage1D: None,
  glCompressedTexSubImage2D: None,
  glCompressedTexSubImage3D: None,
  glCompressedTextureSubImage1D: None,
  glCompressedTextureSubImage2D: None,
  glCompressedTextureSubImage3D: None,
  glCopyBufferSubData: None,
  glCopyImageSubData: None,
  glCopyNamedBufferSubData: None,
  glCopyTexImage1D: None,
  glCopyTexImage2D: None,
  glCopyTexSubImage1D: None,
  glCopyTexSubImage2D: None,
  glCopyTexSubImage3D: None,
  glCopyTextureSubImage1D: None,
  glCopyTextureSubImage2D: None,
  glCopyTextureSubImage3D: None,
  glCreateBuffers: None,
  glCreateFramebuffers: None,
  glCreateProgram: None,
  glCreateProgramPipelines: None,
  glCreateQueries: None,
  glCreateRenderbuffers: None,
  glCreateSamplers: None,
  glCreateShader: None,
  glCreateShaderProgramv: None,
  glCreateTextures: None,
  glCreateTransformFeedbacks: None,
  glCreateVertexArrays: None,
  glCullFace: None,
  glDebugMessageCallback: None,
  glDebugMessageCallbackKHR: None,
  glDebugMessageControl: None,
  glDebugMessageControlKHR: None,
  glDebugMessageInsert: None,
  glDebugMessageInsertKHR: None,
  glDeleteBuffers: None,
  glDeleteFramebuffers: None,
  glDeleteProgram: None,
  glDeleteProgramPipelines: None,
  glDeleteQueries: None,
  glDeleteRenderbuffers: None,
  glDeleteSamplers: None,
  glDeleteShader: None,
  glDeleteSync: None,
  glDeleteTextures: None,
  glDeleteTransformFeedbacks: None,
  glDeleteVertexArrays: None,
  glDepthFunc: None,
  glDepthMask: None,
  glDepthRange: None,
  glDepthRangeArrayv: None,
  glDepthRangeIndexed: None,
  glDepthRangef: None,
  glDetachShader: None,
  glDisable: None,
  glDisableVertexArrayAttrib: None,
  glDisableVertexAttribArray: None,
  glDisablei: None,
  glDispatchCompute: None,
  glDispatchComputeIndirect: None,
  glDrawArrays: None,
  glDrawArraysIndirect: None,
  glDrawArraysInstanced: None,
  glDrawArraysInstancedBaseInstance: None,
  glDrawBuffer: None,
  glDrawBuffers: None,
  glDrawElements: None,
  glDrawElementsBaseVertex: None,
  glDrawElementsIndirect: None,
  glDrawElementsInstanced: None,
  glDrawElementsInstancedBaseInstance: None,
  glDrawElementsInstancedBaseVertex: None,
  glDrawElementsInstancedBaseVertexBaseInstance: None,
  glDrawRangeElements: None,
  glDrawRangeElementsBaseVertex: None,
  glDrawTransformFeedback: None,
  glDrawTransformFeedbackInstanced: None,
  glDrawTransformFeedbackStream: None,
  glDrawTransformFeedbackStreamInstanced: None,
  glEnable: None,
  glEnableVertexArrayAttrib: None,
  glEnableVertexAttribArray: None,
  glEnablei: None,
  glEndConditionalRender: None,
  glEndQuery: None,
  glEndQueryIndexed: None,
  glEndTransformFeedback: None,
  glFenceSync: None,
  glFinish: None,
  glFlush: None,
  glFlushMappedBufferRange: None,
  glFlushMappedNamedBufferRange: None,
  glFramebufferParameteri: None,
  glFramebufferRenderbuffer: None,
  glFramebufferTexture: None,
  glFramebufferTexture1D: None,
  glFramebufferTexture2D: None,
  glFramebufferTexture3D: None,
  glFramebufferTextureLayer: None,
  glFrontFace: None,
  glGenBuffers: None,
  glGenFramebuffers: None,
  glGenProgramPipelines: None,
  glGenQueries: None,
  glGenRenderbuffers: None,
  glGenSamplers: None,
  glGenTextures: None,
  glGenTransformFeedbacks: None,
  glGenVertexArrays: None,
  glGenerateMipmap: None,
  glGenerateTextureMipmap: None,
  glGetActiveAtomicCounterBufferiv: None,
  glGetActiveAttrib: None,
  glGetActiveSubroutineName: None,
  glGetActiveSubroutineUniformName: None,
  glGetActiveSubroutineUniformiv: None,
  glGetActiveUniform: None,
  glGetActiveUniformBlockName: None,
  glGetActiveUniformBlockiv: None,
  glGetActiveUniformName: None,
  glGetActiveUniformsiv: None,
  glGetAttachedShaders: None,
  glGetAttribLocation: None,
  glGetBooleani_v: None,
  glGetBooleanv: None,
  glGetBufferParameteri64v: None,
  glGetBufferParameteriv: None,
  glGetBufferPointerv: None,
  glGetBufferSubData: None,
  glGetCompressedTexImage: None,
  glGetCompressedTextureImage: None,
  glGetCompressedTextureSubImage: None,
  glGetDebugMessageLog: None,
  glGetDebugMessageLogKHR: None,
  glGetDoublei_v: None,
  glGetDoublev: None,
  glGetError: None,
  glGetFloati_v: None,
  glGetFloatv: None,
  glGetFragDataIndex: None,
  glGetFragDataLocation: None,
  glGetFramebufferAttachmentParameteriv: None,
  glGetFramebufferParameteriv: None,
  glGetGraphicsResetStatus: None,
  glGetInteger64i_v: None,
  glGetInteger64v: None,
  glGetIntegeri_v: None,
  glGetIntegerv: None,
  glGetInternalformati64v: None,
  glGetInternalformativ: None,
  glGetMultisamplefv: None,
  glGetNamedBufferParameteri64v: None,
  glGetNamedBufferParameteriv: None,
  glGetNamedBufferPointerv: None,
  glGetNamedBufferSubData: None,
  glGetNamedFramebufferAttachmentParameteriv: None,
  glGetNamedFramebufferParameteriv: None,
  glGetNamedRenderbufferParameteriv: None,
  glGetObjectLabel: None,
  glGetObjectLabelKHR: None,
  glGetObjectPtrLabel: None,
  glGetObjectPtrLabelKHR: None,
  glGetPointerv: None,
  glGetPointervKHR: None,
  glGetProgramBinary: None,
  glGetProgramInfoLog: None,
  glGetProgramInterfaceiv: None,
  glGetProgramPipelineInfoLog: None,
  glGetProgramPipelineiv: None,
  glGetProgramResourceIndex: None,
  glGetProgramResourceLocation: None,
  glGetProgramResourceLocationIndex: None,
  glGetProgramResourceName: None,
  glGetProgramResourceiv: None,
  glGetProgramStageiv: None,
  glGetProgramiv: None,
  glGetQueryBufferObjecti64v: None,
  glGetQueryBufferObjectiv: None,
  glGetQueryBufferObjectui64v: None,
  glGetQueryBufferObjectuiv: None,
  glGetQueryIndexediv: None,
  glGetQueryObjecti64v: None,
  glGetQueryObjectiv: None,
  glGetQueryObjectui64v: None,
  glGetQueryObjectuiv: None,
  glGetQueryiv: None,
  glGetRenderbufferParameteriv: None,
  glGetSamplerParameterIiv: None,
  glGetSamplerParameterIuiv: None,
  glGetSamplerParameterfv: None,
  glGetSamplerParameteriv: None,
  glGetShaderInfoLog: None,
  glGetShaderPrecisionFormat: None,
  glGetShaderSource: None,
  glGetShaderiv: None,
  glGetString: None,
  glGetStringi: None,
  glGetSubroutineIndex: None,
  glGetSubroutineUniformLocation: None,
  glGetSynciv: None,
  glGetTexImage: None,
  glGetTexLevelParameterfv: None,
  glGetTexLevelParameteriv: None,
  glGetTexParameterIiv: None,
  glGetTexParameterIuiv: None,
  glGetTexParameterfv: None,
  glGetTexParameteriv: None,
  glGetTextureImage: None,
  glGetTextureLevelParameterfv: None,
  glGetTextureLevelParameteriv: None,
  glGetTextureParameterIiv: None,
  glGetTextureParameterIuiv: None,
  glGetTextureParameterfv: None,
  glGetTextureParameteriv: None,
  glGetTextureSubImage: None,
  glGetTransformFeedbackVarying: None,
  glGetTransformFeedbacki64_v: None,
  glGetTransformFeedbacki_v: None,
  glGetTransformFeedbackiv: None,
  glGetUniformBlockIndex: None,
  glGetUniformIndices: None,
  glGetUniformLocation: None,
  glGetUniformSubroutineuiv: None,
  glGetUniformdv: None,
  glGetUniformfv: None,
  glGetUniformiv: None,
  glGetUniformuiv: None,
  glGetVertexArrayIndexed64iv: None,
  glGetVertexArrayIndexediv: None,
  glGetVertexArrayiv: None,
  glGetVertexAttribIiv: None,
  glGetVertexAttribIuiv: None,
  glGetVertexAttribLdv: None,
  glGetVertexAttribPointerv: None,
  glGetVertexAttribdv: None,
  glGetVertexAttribfv: None,
  glGetVertexAttribiv: None,
  glGetnColorTable: None,
  glGetnCompressedTexImage: None,
  glGetnConvolutionFilter: None,
  glGetnHistogram: None,
  glGetnMapdv: None,
  glGetnMapfv: None,
  glGetnMapiv: None,
  glGetnMinmax: None,
  glGetnPixelMapfv: None,
  glGetnPixelMapuiv: None,
  glGetnPixelMapusv: None,
  glGetnPolygonStipple: None,
  glGetnSeparableFilter: None,
  glGetnTexImage: None,
  glGetnUniformdv: None,
  glGetnUniformfv: None,
  glGetnUniformiv: None,
  glGetnUniformuiv: None,
  glHint: None,
  glInvalidateBufferData: None,
  glInvalidateBufferSubData: None,
  glInvalidateFramebuffer: None,
  glInvalidateNamedFramebufferData: None,
  glInvalidateNamedFramebufferSubData: None,
  glInvalidateSubFramebuffer: None,
  glInvalidateTexImage: None,
  glInvalidateTexSubImage: None,
  glIsBuffer: None,
  glIsEnabled: None,
  glIsEnabledi: None,
  glIsFramebuffer: None,
  glIsProgram: None,
  glIsProgramPipeline: None,
  glIsQuery: None,
  glIsRenderbuffer: None,
  glIsSampler: None,
  glIsShader: None,
  glIsSync: None,
  glIsTexture: None,
  glIsTransformFeedback: None,
  glIsVertexArray: None,
  glLineWidth: None,
  glLinkProgram: None,
  glLogicOp: None,
  glMapBuffer: None,
  glMapBufferRange: None,
  glMapNamedBuffer: None,
  glMapNamedBufferRange: None,
  glMemoryBarrier: None,
  glMemoryBarrierByRegion: None,
  glMinSampleShading: None,
  glMultiDrawArrays: None,
  glMultiDrawArraysIndirect: None,
  glMultiDrawArraysIndirectCount: None,
  glMultiDrawElements: None,
  glMultiDrawElementsBaseVertex: None,
  glMultiDrawElementsIndirect: None,
  glMultiDrawElementsIndirectCount: None,
  glMultiTexCoordP1ui: None,
  glMultiTexCoordP1uiv: None,
  glMultiTexCoordP2ui: None,
  glMultiTexCoordP2uiv: None,
  glMultiTexCoordP3ui: None,
  glMultiTexCoordP3uiv: None,
  glMultiTexCoordP4ui: None,
  glMultiTexCoordP4uiv: None,
  glNamedBufferData: None,
  glNamedBufferStorage: None,
  glNamedBufferSubData: None,
  glNamedFramebufferDrawBuffer: None,
  glNamedFramebufferDrawBuffers: None,
  glNamedFramebufferParameteri: None,
  glNamedFramebufferReadBuffer: None,
  glNamedFramebufferRenderbuffer: None,
  glNamedFramebufferTexture: None,
  glNamedFramebufferTextureLayer: None,
  glNamedRenderbufferStorage: None,
  glNamedRenderbufferStorageMultisample: None,
  glNormalP3ui: None,
  glNormalP3uiv: None,
  glObjectLabel: None,
  glObjectLabelKHR: None,
  glObjectPtrLabel: None,
  glObjectPtrLabelKHR: None,
  glPatchParameterfv: None,
  glPatchParameteri: None,
  glPauseTransformFeedback: None,
  glPixelStoref: None,
  glPixelStorei: None,
  glPointParameterf: None,
  glPointParameterfv: None,
  glPointParameteri: None,
  glPointParameteriv: None,
  glPointSize: None,
  glPolygonMode: None,
  glPolygonOffset: None,
  glPolygonOffsetClamp: None,
  glPopDebugGroup: None,
  glPopDebugGroupKHR: None,
  glPrimitiveBoundingBox: None,
  glPrimitiveRestartIndex: None,
  glProgramBinary: None,
  glProgramParameteri: None,
  glProgramUniform1d: None,
  glProgramUniform1dv: None,
  glProgramUniform1f: None,
  glProgramUniform1fv: None,
  glProgramUniform1i: None,
  glProgramUniform1iv: None,
  glProgramUniform1ui: None,
  glProgramUniform1uiv: None,
  glProgramUniform2d: None,
  glProgramUniform2dv: None,
  glProgramUniform2f: None,
  glProgramUniform2fv: None,
  glProgramUniform2i: None,
  glProgramUniform2iv: None,
  glProgramUniform2ui: None,
  glProgramUniform2uiv: None,
  glProgramUniform3d: None,
  glProgramUniform3dv: None,
  glProgramUniform3f: None,
  glProgramUniform3fv: None,
  glProgramUniform3i: None,
  glProgramUniform3iv: None,
  glProgramUniform3ui: None,
  glProgramUniform3uiv: None,
  glProgramUniform4d: None,
  glProgramUniform4dv: None,
  glProgramUniform4f: None,
  glProgramUniform4fv: None,
  glProgramUniform4i: None,
  glProgramUniform4iv: None,
  glProgramUniform4ui: None,
  glProgramUniform4uiv: None,
  glProgramUniformMatrix2dv: None,
  glProgramUniformMatrix2fv: None,
  glProgramUniformMatrix2x3dv: None,
  glProgramUniformMatrix2x3fv: None,
  glProgramUniformMatrix2x4dv: None,
  glProgramUniformMatrix2x4fv: None,
  glProgramUniformMatrix3dv: None,
  glProgramUniformMatrix3fv: None,
  glProgramUniformMatrix3x2dv: None,
  glProgramUniformMatrix3x2fv: None,
  glProgramUniformMatrix3x4dv: None,
  glProgramUniformMatrix3x4fv: None,
  glProgramUniformMatrix4dv: None,
  glProgramUniformMatrix4fv: None,
  glProgramUniformMatrix4x2dv: None,
  glProgramUniformMatrix4x2fv: None,
  glProgramUniformMatrix4x3dv: None,
  glProgramUniformMatrix4x3fv: None,
  glProvokingVertex: None,
  glPushDebugGroup: None,
  glPushDebugGroupKHR: None,
  glQueryCounter: None,
  glReadBuffer: None,
  glReadPixels: None,
  glReadnPixels: None,
  glReleaseShaderCompiler: None,
  glRenderbufferStorage: None,
  glRenderbufferStorageMultisample: None,
  glResumeTransformFeedback: None,
  glSampleCoverage: None,
  glSampleMaski: None,
  glSamplerParameterIiv: None,
  glSamplerParameterIuiv: None,
  glSamplerParameterf: None,
  glSamplerParameterfv: None,
  glSamplerParameteri: None,
  glSamplerParameteriv: None,
  glScissor: None,
  glScissorArrayv: None,
  glScissorIndexed: None,
  glScissorIndexedv: None,
  glSecondaryColorP3ui: None,
  glSecondaryColorP3uiv: None,
  glShaderBinary: None,
  glShaderSource: None,
  glShaderStorageBlockBinding: None,
  glSpecializeShader: None,
  glStencilFunc: None,
  glStencilFuncSeparate: None,
  glStencilMask: None,
  glStencilMaskSeparate: None,
  glStencilOp: None,
  glStencilOpSeparate: None,
  glTexBuffer: None,
  glTexBufferRange: None,
  glTexCoordP1ui: None,
  glTexCoordP1uiv: None,
  glTexCoordP2ui: None,
  glTexCoordP2uiv: None,
  glTexCoordP3ui: None,
  glTexCoordP3uiv: None,
  glTexCoordP4ui: None,
  glTexCoordP4uiv: None,
  glTexImage1D: None,
  glTexImage2D: None,
  glTexImage2DMultisample: None,
  glTexImage3D: None,
  glTexImage3DMultisample: None,
  glTexParameterIiv: None,
  glTexParameterIuiv: None,
  glTexParameterf: None,
  glTexParameterfv: None,
  glTexParameteri: None,
  glTexParameteriv: None,
  glTexStorage1D: None,
  glTexStorage2D: None,
  glTexStorage2DMultisample: None,
  glTexStorage3D: None,
  glTexStorage3DMultisample: None,
  glTexSubImage1D: None,
  glTexSubImage2D: None,
  glTexSubImage3D: None,
  glTextureBarrier: None,
  glTextureBuffer: None,
  glTextureBufferRange: None,
  glTextureParameterIiv: None,
  glTextureParameterIuiv: None,
  glTextureParameterf: None,
  glTextureParameterfv: None,
  glTextureParameteri: None,
  glTextureParameteriv: None,
  glTextureStorage1D: None,
  glTextureStorage2D: None,
  glTextureStorage2DMultisample: None,
  glTextureStorage3D: None,
  glTextureStorage3DMultisample: None,
  glTextureSubImage1D: None,
  glTextureSubImage2D: None,
  glTextureSubImage3D: None,
  glTextureView: None,
  glTransformFeedbackBufferBase: None,
  glTransformFeedbackBufferRange: None,
  glTransformFeedbackVaryings: None,
  glUniform1d: None,
  glUniform1dv: None,
  glUniform1f: None,
  glUniform1fv: None,
  glUniform1i: None,
  glUniform1iv: None,
  glUniform1ui: None,
  glUniform1uiv: None,
  glUniform2d: None,
  glUniform2dv: None,
  glUniform2f: None,
  glUniform2fv: None,
  glUniform2i: None,
  glUniform2iv: None,
  glUniform2ui: None,
  glUniform2uiv: None,
  glUniform3d: None,
  glUniform3dv: None,
  glUniform3f: None,
  glUniform3fv: None,
  glUniform3i: None,
  glUniform3iv: None,
  glUniform3ui: None,
  glUniform3uiv: None,
  glUniform4d: None,
  glUniform4dv: None,
  glUniform4f: None,
  glUniform4fv: None,
  glUniform4i: None,
  glUniform4iv: None,
  glUniform4ui: None,
  glUniform4uiv: None,
  glUniformBlockBinding: None,
  glUniformMatrix2dv: None,
  glUniformMatrix2fv: None,
  glUniformMatrix2x3dv: None,
  glUniformMatrix2x3fv: None,
  glUniformMatrix2x4dv: None,
  glUniformMatrix2x4fv: None,
  glUniformMatrix3dv: None,
  glUniformMatrix3fv: None,
  glUniformMatrix3x2dv: None,
  glUniformMatrix3x2fv: None,
  glUniformMatrix3x4dv: None,
  glUniformMatrix3x4fv: None,
  glUniformMatrix4dv: None,
  glUniformMatrix4fv: None,
  glUniformMatrix4x2dv: None,
  glUniformMatrix4x2fv: None,
  glUniformMatrix4x3dv: None,
  glUniformMatrix4x3fv: None,
  glUniformSubroutinesuiv: None,
  glUnmapBuffer: None,
  glUnmapNamedBuffer: None,
  glUseProgram: None,
  glUseProgramStages: None,
  glValidateProgram: None,
  glValidateProgramPipeline: None,
  glVertexArrayAttribBinding: None,
  glVertexArrayAttribFormat: None,
  glVertexArrayAttribIFormat: None,
  glVertexArrayAttribLFormat: None,
  glVertexArrayBindingDivisor: None,
  glVertexArrayElementBuffer: None,
  glVertexArrayVertexBuffer: None,
  glVertexArrayVertexBuffers: None,
  glVertexAttrib1d: None,
  glVertexAttrib1dv: None,
  glVertexAttrib1f: None,
  glVertexAttrib1fv: None,
  glVertexAttrib1s: None,
  glVertexAttrib1sv: None,
  glVertexAttrib2d: None,
  glVertexAttrib2dv: None,
  glVertexAttrib2f: None,
  glVertexAttrib2fv: None,
  glVertexAttrib2s: None,
  glVertexAttrib2sv: None,
  glVertexAttrib3d: None,
  glVertexAttrib3dv: None,
  glVertexAttrib3f: None,
  glVertexAttrib3fv: None,
  glVertexAttrib3s: None,
  glVertexAttrib3sv: None,
  glVertexAttrib4Nbv: None,
  glVertexAttrib4Niv: None,
  glVertexAttrib4Nsv: None,
  glVertexAttrib4Nub: None,
  glVertexAttrib4Nubv: None,
  glVertexAttrib4Nuiv: None,
  glVertexAttrib4Nusv: None,
  glVertexAttrib4bv: None,
  glVertexAttrib4d: None,
  glVertexAttrib4dv: None,
  glVertexAttrib4f: None,
  glVertexAttrib4fv: None,
  glVertexAttrib4iv: None,
  glVertexAttrib4s: None,
  glVertexAttrib4sv: None,
  glVertexAttrib4ubv: None,
  glVertexAttrib4uiv: None,
  glVertexAttrib4usv: None,
  glVertexAttribBinding: None,
  glVertexAttribDivisor: None,
  glVertexAttribFormat: None,
  glVertexAttribI1i: None,
  glVertexAttribI1iv: None,
  glVertexAttribI1ui: None,
  glVertexAttribI1uiv: None,
  glVertexAttribI2i: None,
  glVertexAttribI2iv: None,
  glVertexAttribI2ui: None,
  glVertexAttribI2uiv: None,
  glVertexAttribI3i: None,
  glVertexAttribI3iv: None,
  glVertexAttribI3ui: None,
  glVertexAttribI3uiv: None,
  glVertexAttribI4bv: None,
  glVertexAttribI4i: None,
  glVertexAttribI4iv: None,
  glVertexAttribI4sv: None,
  glVertexAttribI4ubv: None,
  glVertexAttribI4ui: None,
  glVertexAttribI4uiv: None,
  glVertexAttribI4usv: None,
  glVertexAttribIFormat: None,
  glVertexAttribIPointer: None,
  glVertexAttribL1d: None,
  glVertexAttribL1dv: None,
  glVertexAttribL2d: None,
  glVertexAttribL2dv: None,
  glVertexAttribL3d: None,
  glVertexAttribL3dv: None,
  glVertexAttribL4d: None,
  glVertexAttribL4dv: None,
  glVertexAttribLFormat: None,
  glVertexAttribLPointer: None,
  glVertexAttribP1ui: None,
  glVertexAttribP1uiv: None,
  glVertexAttribP2ui: None,
  glVertexAttribP2uiv: None,
  glVertexAttribP3ui: None,
  glVertexAttribP3uiv: None,
  glVertexAttribP4ui: None,
  glVertexAttribP4uiv: None,
  glVertexAttribPointer: None,
  glVertexBindingDivisor: None,
  glVertexP2ui: None,
  glVertexP2uiv: None,
  glVertexP3ui: None,
  glVertexP3uiv: None,
  glVertexP4ui: None,
  glVertexP4uiv: None,
  glViewport: None,
  glViewportArrayv: None,
  glViewportIndexedf: None,
  glViewportIndexedfv: None,
  glWaitSync: None,
};
