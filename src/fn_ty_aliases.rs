use super::*;

pub type glActiveShaderProgram_t = unsafe extern "system" fn(pipeline: GLuint, program: GLuint);
pub type glActiveTexture_t = unsafe extern "system" fn(texture: GLenum);
pub type glAttachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);
pub type glBeginConditionalRender_t = unsafe extern "system" fn(id: GLuint, mode: GLenum);
pub type glBeginQuery_t = unsafe extern "system" fn(target: GLenum, id: GLuint);
pub type glBeginQueryIndexed_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, id: GLuint);
pub type glBeginTransformFeedback_t = unsafe extern "system" fn(primitiveMode: GLenum);
pub type glBindAttribLocation_t =
  unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar);
pub type glBindBuffer_t = unsafe extern "system" fn(target: GLenum, buffer: GLuint);
pub type glBindBufferBase_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint);
pub type glBindBufferRange_t = unsafe extern "system" fn(
  target: GLenum,
  index: GLuint,
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
);
pub type glBindBuffersBase_t =
  unsafe extern "system" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint);
pub type glBindBuffersRange_t = unsafe extern "system" fn(
  target: GLenum,
  first: GLuint,
  count: GLsizei,
  buffers: *const GLuint,
  offsets: *const GLintptr,
  sizes: *const GLsizeiptr,
);
pub type glBindFragDataLocation_t =
  unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);
pub type glBindFragDataLocationIndexed_t = unsafe extern "system" fn(
  program: GLuint,
  colorNumber: GLuint,
  index: GLuint,
  name: *const GLchar,
);
pub type glBindFramebuffer_t = unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);
pub type glBindImageTexture_t = unsafe extern "system" fn(
  unit: GLuint,
  texture: GLuint,
  level: GLint,
  layered: GLboolean,
  layer: GLint,
  access: GLenum,
  format: GLenum,
);
pub type glBindImageTextures_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);
pub type glBindProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);
pub type glBindRenderbuffer_t = unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint);
pub type glBindSampler_t = unsafe extern "system" fn(unit: GLuint, sampler: GLuint);
pub type glBindSamplers_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, samplers: *const GLuint);
pub type glBindTexture_t = unsafe extern "system" fn(target: GLenum, texture: GLuint);
pub type glBindTextureUnit_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint);
pub type glBindTextures_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);
pub type glBindTransformFeedback_t = unsafe extern "system" fn(target: GLenum, id: GLuint);
pub type glBindVertexArray_t = unsafe extern "system" fn(array: GLuint);
pub type glBindVertexBuffer_t = unsafe extern "system" fn(
  bindingindex: GLuint,
  buffer: GLuint,
  offset: GLintptr,
  stride: GLsizei,
);
pub type glBindVertexBuffers_t = unsafe extern "system" fn(
  first: GLuint,
  count: GLsizei,
  buffers: *const GLuint,
  offsets: *const GLintptr,
  strides: *const GLsizei,
);
pub type glBlendBarrier_t = unsafe extern "system" fn();
pub type glBlendColor_t =
  unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
pub type glBlendEquation_t = unsafe extern "system" fn(mode: GLenum);
pub type glBlendEquationSeparate_t = unsafe extern "system" fn(modeRGB: GLenum, modeAlpha: GLenum);
pub type glBlendEquationSeparatei_t =
  unsafe extern "system" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
pub type glBlendEquationi_t = unsafe extern "system" fn(buf: GLuint, mode: GLenum);
pub type glBlendFunc_t = unsafe extern "system" fn(sfactor: GLenum, dfactor: GLenum);
pub type glBlendFuncSeparate_t = unsafe extern "system" fn(
  sfactorRGB: GLenum,
  dfactorRGB: GLenum,
  sfactorAlpha: GLenum,
  dfactorAlpha: GLenum,
);
pub type glBlendFuncSeparatei_t = unsafe extern "system" fn(
  buf: GLuint,
  srcRGB: GLenum,
  dstRGB: GLenum,
  srcAlpha: GLenum,
  dstAlpha: GLenum,
);
pub type glBlendFunci_t = unsafe extern "system" fn(buf: GLuint, src: GLenum, dst: GLenum);
pub type glBlitFramebuffer_t = unsafe extern "system" fn(
  srcX0: GLint,
  srcY0: GLint,
  srcX1: GLint,
  srcY1: GLint,
  dstX0: GLint,
  dstY0: GLint,
  dstX1: GLint,
  dstY1: GLint,
  mask: GLbitfield,
  filter: GLenum,
);
pub type glBlitNamedFramebuffer_t = unsafe extern "system" fn(
  readFramebuffer: GLuint,
  drawFramebuffer: GLuint,
  srcX0: GLint,
  srcY0: GLint,
  srcX1: GLint,
  srcY1: GLint,
  dstX0: GLint,
  dstY0: GLint,
  dstX1: GLint,
  dstY1: GLint,
  mask: GLbitfield,
  filter: GLenum,
);
pub type glBufferData_t =
  unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
pub type glBufferStorage_t = unsafe extern "system" fn(
  target: GLenum,
  size: GLsizeiptr,
  data: *const c_void,
  flags: GLbitfield,
);
pub type glBufferSubData_t = unsafe extern "system" fn(
  target: GLenum,
  offset: GLintptr,
  size: GLsizeiptr,
  data: *const c_void,
);
pub type glCheckFramebufferStatus_t = unsafe extern "system" fn(target: GLenum) -> GLenum;
pub type glCheckNamedFramebufferStatus_t =
  unsafe extern "system" fn(framebuffer: GLuint, target: GLenum) -> GLenum;
pub type glClampColor_t = unsafe extern "system" fn(target: GLenum, clamp: GLenum);
pub type glClear_t = unsafe extern "system" fn(mask: GLbitfield);
pub type glClearBufferData_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
pub type glClearBufferSubData_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  offset: GLintptr,
  size: GLsizeiptr,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
pub type glClearBufferfi_t =
  unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
pub type glClearBufferfv_t =
  unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
pub type glClearBufferiv_t =
  unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
pub type glClearBufferuiv_t =
  unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
pub type glClearColor_t =
  unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
pub type glClearDepth_t = unsafe extern "system" fn(depth: GLdouble);
pub type glClearDepthf_t = unsafe extern "system" fn(d: GLfloat);
pub type glClearNamedBufferData_t = unsafe extern "system" fn(
  buffer: GLuint,
  internalformat: GLenum,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
pub type glClearNamedBufferSubData_t = unsafe extern "system" fn(
  buffer: GLuint,
  internalformat: GLenum,
  offset: GLintptr,
  size: GLsizeiptr,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
pub type glClearNamedFramebufferfi_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  buffer: GLenum,
  drawbuffer: GLint,
  depth: GLfloat,
  stencil: GLint,
);
pub type glClearNamedFramebufferfv_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  buffer: GLenum,
  drawbuffer: GLint,
  value: *const GLfloat,
);
pub type glClearNamedFramebufferiv_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  buffer: GLenum,
  drawbuffer: GLint,
  value: *const GLint,
);
pub type glClearNamedFramebufferuiv_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  buffer: GLenum,
  drawbuffer: GLint,
  value: *const GLuint,
);
pub type glClearStencil_t = unsafe extern "system" fn(s: GLint);
pub type glClearTexImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
pub type glClearTexSubImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  ty: GLenum,
  data: *const c_void,
);
pub type glClientWaitSync_t =
  unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
pub type glClipControl_t = unsafe extern "system" fn(origin: GLenum, depth: GLenum);
pub type glColorMask_t =
  unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
pub type glColorMaski_t =
  unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
pub type glColorP3ui_t = unsafe extern "system" fn(ty: GLenum, color: GLuint);
pub type glColorP3uiv_t = unsafe extern "system" fn(ty: GLenum, color: *const GLuint);
pub type glColorP4ui_t = unsafe extern "system" fn(ty: GLenum, color: GLuint);
pub type glColorP4uiv_t = unsafe extern "system" fn(ty: GLenum, color: *const GLuint);
pub type glCompileShader_t = unsafe extern "system" fn(shader: GLuint);
pub type glCompressedTexImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  width: GLsizei,
  border: GLint,
  imageSize: GLsizei,
  data: *const c_void,
);
pub type glCompressedTexImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  border: GLint,
  imageSize: GLsizei,
  data: *const c_void,
);
pub type glCompressedTexImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  border: GLint,
  imageSize: GLsizei,
  data: *const c_void,
);
pub type glCompressedTexSubImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  width: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
pub type glCompressedTexSubImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
pub type glCompressedTexSubImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
pub type glCompressedTextureSubImage1D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  width: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
pub type glCompressedTextureSubImage2D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
pub type glCompressedTextureSubImage3D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  imageSize: GLsizei,
  data: *const c_void,
);
pub type glCopyBufferSubData_t = unsafe extern "system" fn(
  readTarget: GLenum,
  writeTarget: GLenum,
  readOffset: GLintptr,
  writeOffset: GLintptr,
  size: GLsizeiptr,
);
pub type glCopyImageSubData_t = unsafe extern "system" fn(
  srcName: GLuint,
  srcTarget: GLenum,
  srcLevel: GLint,
  srcX: GLint,
  srcY: GLint,
  srcZ: GLint,
  dstName: GLuint,
  dstTarget: GLenum,
  dstLevel: GLint,
  dstX: GLint,
  dstY: GLint,
  dstZ: GLint,
  srcWidth: GLsizei,
  srcHeight: GLsizei,
  srcDepth: GLsizei,
);
pub type glCopyNamedBufferSubData_t = unsafe extern "system" fn(
  readBuffer: GLuint,
  writeBuffer: GLuint,
  readOffset: GLintptr,
  writeOffset: GLintptr,
  size: GLsizeiptr,
);
pub type glCopyTexImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  x: GLint,
  y: GLint,
  width: GLsizei,
  border: GLint,
);
pub type glCopyTexImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLenum,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
  border: GLint,
);
pub type glCopyTexSubImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
);
pub type glCopyTexSubImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
pub type glCopyTexSubImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
pub type glCopyTextureSubImage1D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
);
pub type glCopyTextureSubImage2D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
pub type glCopyTextureSubImage3D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
pub type glCreateBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);
pub type glCreateFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);
pub type glCreateProgram_t = unsafe extern "system" fn() -> GLuint;
pub type glCreateProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);
pub type glCreateQueries_t =
  unsafe extern "system" fn(target: GLenum, n: GLsizei, ids: *mut GLuint);
pub type glCreateRenderbuffers_t =
  unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);
pub type glCreateSamplers_t = unsafe extern "system" fn(n: GLsizei, samplers: *mut GLuint);
pub type glCreateShader_t = unsafe extern "system" fn(ty: GLenum) -> GLuint;
pub type glCreateShaderProgramv_t =
  unsafe extern "system" fn(ty: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint;
pub type glCreateTextures_t =
  unsafe extern "system" fn(target: GLenum, n: GLsizei, textures: *mut GLuint);
pub type glCreateTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
pub type glCreateVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);
pub type glCullFace_t = unsafe extern "system" fn(mode: GLenum);
pub type glDebugMessageCallback_t =
  unsafe extern "system" fn(callback: GLDEBUGPROC, userParam: *const c_void);
pub type glDebugMessageCallbackKHR_t =
  unsafe extern "system" fn(callback: GLDEBUGPROCKHR, userParam: *const c_void);
pub type glDebugMessageControl_t = unsafe extern "system" fn(
  source: GLenum,
  ty: GLenum,
  severity: GLenum,
  count: GLsizei,
  ids: *const GLuint,
  enabled: GLboolean,
);
pub type glDebugMessageControlKHR_t = unsafe extern "system" fn(
  source: GLenum,
  ty: GLenum,
  severity: GLenum,
  count: GLsizei,
  ids: *const GLuint,
  enabled: GLboolean,
);
pub type glDebugMessageInsert_t = unsafe extern "system" fn(
  source: GLenum,
  ty: GLenum,
  id: GLuint,
  severity: GLenum,
  length: GLsizei,
  buf: *const GLchar,
);
pub type glDebugMessageInsertKHR_t = unsafe extern "system" fn(
  source: GLenum,
  ty: GLenum,
  id: GLuint,
  severity: GLenum,
  length: GLsizei,
  buf: *const GLchar,
);
pub type glDeleteBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);
pub type glDeleteFramebuffers_t =
  unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);
pub type glDeleteProgram_t = unsafe extern "system" fn(program: GLuint);
pub type glDeleteProgramPipelines_t =
  unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);
pub type glDeleteQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);
pub type glDeleteRenderbuffers_t =
  unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);
pub type glDeleteSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint);
pub type glDeleteShader_t = unsafe extern "system" fn(shader: GLuint);
pub type glDeleteSync_t = unsafe extern "system" fn(sync: GLsync);
pub type glDeleteTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);
pub type glDeleteTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);
pub type glDeleteVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);
pub type glDepthFunc_t = unsafe extern "system" fn(func: GLenum);
pub type glDepthMask_t = unsafe extern "system" fn(flag: GLboolean);
pub type glDepthRange_t = unsafe extern "system" fn(n: GLdouble, f: GLdouble);
pub type glDepthRangeArrayv_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);
pub type glDepthRangeIndexed_t = unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);
pub type glDepthRangef_t = unsafe extern "system" fn(n: GLfloat, f: GLfloat);
pub type glDetachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);
pub type glDisable_t = unsafe extern "system" fn(cap: GLenum);
pub type glDisableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);
pub type glDisableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);
pub type glDisablei_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
pub type glDispatchCompute_t =
  unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);
pub type glDispatchComputeIndirect_t = unsafe extern "system" fn(indirect: GLintptr);
pub type glDrawArrays_t = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei);
pub type glDrawArraysIndirect_t = unsafe extern "system" fn(mode: GLenum, indirect: *const c_void);
pub type glDrawArraysInstanced_t =
  unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
pub type glDrawArraysInstancedBaseInstance_t = unsafe extern "system" fn(
  mode: GLenum,
  first: GLint,
  count: GLsizei,
  instancecount: GLsizei,
  baseinstance: GLuint,
);
pub type glDrawBuffer_t = unsafe extern "system" fn(buf: GLenum);
pub type glDrawBuffers_t = unsafe extern "system" fn(n: GLsizei, bufs: *const GLenum);
pub type glDrawElements_t =
  unsafe extern "system" fn(mode: GLenum, count: GLsizei, ty: GLenum, indices: *const c_void);
pub type glDrawElementsBaseVertex_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  basevertex: GLint,
);
pub type glDrawElementsIndirect_t =
  unsafe extern "system" fn(mode: GLenum, ty: GLenum, indirect: *const c_void);
pub type glDrawElementsInstanced_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  instancecount: GLsizei,
);
pub type glDrawElementsInstancedBaseInstance_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  instancecount: GLsizei,
  baseinstance: GLuint,
);
pub type glDrawElementsInstancedBaseVertex_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  instancecount: GLsizei,
  basevertex: GLint,
);
pub type glDrawElementsInstancedBaseVertexBaseInstance_t = unsafe extern "system" fn(
  mode: GLenum,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  instancecount: GLsizei,
  basevertex: GLint,
  baseinstance: GLuint,
);
pub type glDrawRangeElements_t = unsafe extern "system" fn(
  mode: GLenum,
  start: GLuint,
  end: GLuint,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
);
pub type glDrawRangeElementsBaseVertex_t = unsafe extern "system" fn(
  mode: GLenum,
  start: GLuint,
  end: GLuint,
  count: GLsizei,
  ty: GLenum,
  indices: *const c_void,
  basevertex: GLint,
);
pub type glDrawTransformFeedback_t = unsafe extern "system" fn(mode: GLenum, id: GLuint);
pub type glDrawTransformFeedbackInstanced_t =
  unsafe extern "system" fn(mode: GLenum, id: GLuint, instancecount: GLsizei);
pub type glDrawTransformFeedbackStream_t =
  unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint);
pub type glDrawTransformFeedbackStreamInstanced_t =
  unsafe extern "system" fn(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei);
pub type glEnable_t = unsafe extern "system" fn(cap: GLenum);
pub type glEnableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);
pub type glEnableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);
pub type glEnablei_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
pub type glEndConditionalRender_t = unsafe extern "system" fn();
pub type glEndQuery_t = unsafe extern "system" fn(target: GLenum);
pub type glEndQueryIndexed_t = unsafe extern "system" fn(target: GLenum, index: GLuint);
pub type glEndTransformFeedback_t = unsafe extern "system" fn();
pub type glFenceSync_t = unsafe extern "system" fn(condition: GLenum, flags: GLbitfield) -> GLsync;
pub type glFinish_t = unsafe extern "system" fn();
pub type glFlush_t = unsafe extern "system" fn();
pub type glFlushMappedBufferRange_t =
  unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr);
pub type glFlushMappedNamedBufferRange_t =
  unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
pub type glFramebufferParameteri_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);
pub type glFramebufferRenderbuffer_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  renderbuffertarget: GLenum,
  renderbuffer: GLuint,
);
pub type glFramebufferTexture_t =
  unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
pub type glFramebufferTexture1D_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  textarget: GLenum,
  texture: GLuint,
  level: GLint,
);
pub type glFramebufferTexture2D_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  textarget: GLenum,
  texture: GLuint,
  level: GLint,
);
pub type glFramebufferTexture3D_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  textarget: GLenum,
  texture: GLuint,
  level: GLint,
  zoffset: GLint,
);
pub type glFramebufferTextureLayer_t = unsafe extern "system" fn(
  target: GLenum,
  attachment: GLenum,
  texture: GLuint,
  level: GLint,
  layer: GLint,
);
pub type glFrontFace_t = unsafe extern "system" fn(mode: GLenum);
pub type glGenBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);
pub type glGenFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);
pub type glGenProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);
pub type glGenQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
pub type glGenRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);
pub type glGenSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint);
pub type glGenTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);
pub type glGenTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);
pub type glGenVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);
pub type glGenerateMipmap_t = unsafe extern "system" fn(target: GLenum);
pub type glGenerateTextureMipmap_t = unsafe extern "system" fn(texture: GLuint);
pub type glGetActiveAtomicCounterBufferiv_t = unsafe extern "system" fn(
  program: GLuint,
  bufferIndex: GLuint,
  pname: GLenum,
  params: *mut GLint,
);
pub type glGetActiveAttrib_t = unsafe extern "system" fn(
  program: GLuint,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  size: *mut GLint,
  ty: *mut GLenum,
  name: *mut GLchar,
);
pub type glGetActiveSubroutineName_t = unsafe extern "system" fn(
  program: GLuint,
  shadertype: GLenum,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  name: *mut GLchar,
);
pub type glGetActiveSubroutineUniformName_t = unsafe extern "system" fn(
  program: GLuint,
  shadertype: GLenum,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  name: *mut GLchar,
);
pub type glGetActiveSubroutineUniformiv_t = unsafe extern "system" fn(
  program: GLuint,
  shadertype: GLenum,
  index: GLuint,
  pname: GLenum,
  values: *mut GLint,
);
pub type glGetActiveUniform_t = unsafe extern "system" fn(
  program: GLuint,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  size: *mut GLint,
  ty: *mut GLenum,
  name: *mut GLchar,
);
pub type glGetActiveUniformBlockName_t = unsafe extern "system" fn(
  program: GLuint,
  uniformBlockIndex: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  uniformBlockName: *mut GLchar,
);
pub type glGetActiveUniformBlockiv_t = unsafe extern "system" fn(
  program: GLuint,
  uniformBlockIndex: GLuint,
  pname: GLenum,
  params: *mut GLint,
);
pub type glGetActiveUniformName_t = unsafe extern "system" fn(
  program: GLuint,
  uniformIndex: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  uniformName: *mut GLchar,
);
pub type glGetActiveUniformsiv_t = unsafe extern "system" fn(
  program: GLuint,
  uniformCount: GLsizei,
  uniformIndices: *const GLuint,
  pname: GLenum,
  params: *mut GLint,
);
pub type glGetAttachedShaders_t = unsafe extern "system" fn(
  program: GLuint,
  maxCount: GLsizei,
  count: *mut GLsizei,
  shaders: *mut GLuint,
);
pub type glGetAttribLocation_t =
  unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type glGetBooleani_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLboolean);
pub type glGetBooleanv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLboolean);
pub type glGetBufferParameteri64v_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint64);
pub type glGetBufferParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type glGetBufferPointerv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut c_void);
pub type glGetBufferSubData_t =
  unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
pub type glGetCompressedTexImage_t =
  unsafe extern "system" fn(target: GLenum, level: GLint, img: *mut c_void);
pub type glGetCompressedTextureImage_t =
  unsafe extern "system" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void);
pub type glGetCompressedTextureSubImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  bufSize: GLsizei,
  pixels: *mut c_void,
);
pub type glGetDebugMessageLog_t = unsafe extern "system" fn(
  count: GLuint,
  bufSize: GLsizei,
  sources: *mut GLenum,
  types: *mut GLenum,
  ids: *mut GLuint,
  severities: *mut GLenum,
  lengths: *mut GLsizei,
  messageLog: *mut GLchar,
) -> GLuint;
pub type glGetDebugMessageLogKHR_t = unsafe extern "system" fn(
  count: GLuint,
  bufSize: GLsizei,
  sources: *mut GLenum,
  types: *mut GLenum,
  ids: *mut GLuint,
  severities: *mut GLenum,
  lengths: *mut GLsizei,
  messageLog: *mut GLchar,
) -> GLuint;
pub type glGetDoublei_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLdouble);
pub type glGetDoublev_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLdouble);
pub type glGetError_t = unsafe extern "system" fn() -> GLenum;
pub type glGetFloati_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLfloat);
pub type glGetFloatv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLfloat);
pub type glGetFragDataIndex_t =
  unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type glGetFragDataLocation_t =
  unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type glGetFramebufferAttachmentParameteriv_t =
  unsafe extern "system" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
pub type glGetFramebufferParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type glGetGraphicsResetStatus_t = unsafe extern "system" fn() -> GLenum;
pub type glGetInteger64i_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint64);
pub type glGetInteger64v_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLint64);
pub type glGetIntegeri_v_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint);
pub type glGetIntegerv_t = unsafe extern "system" fn(pname: GLenum, data: *mut GLint);
pub type glGetInternalformati64v_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  pname: GLenum,
  count: GLsizei,
  params: *mut GLint64,
);
pub type glGetInternalformativ_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  pname: GLenum,
  count: GLsizei,
  params: *mut GLint,
);
pub type glGetMultisamplefv_t =
  unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *mut GLfloat);
pub type glGetNamedBufferParameteri64v_t =
  unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint64);
pub type glGetNamedBufferParameteriv_t =
  unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetNamedBufferPointerv_t =
  unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut *mut c_void);
pub type glGetNamedBufferSubData_t =
  unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
pub type glGetNamedFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  attachment: GLenum,
  pname: GLenum,
  params: *mut GLint,
);
pub type glGetNamedFramebufferParameteriv_t =
  unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: *mut GLint);
pub type glGetNamedRenderbufferParameteriv_t =
  unsafe extern "system" fn(renderbuffer: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetObjectLabel_t = unsafe extern "system" fn(
  identifier: GLenum,
  name: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  label: *mut GLchar,
);
pub type glGetObjectLabelKHR_t = unsafe extern "system" fn(
  identifier: GLenum,
  name: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  label: *mut GLchar,
);
pub type glGetObjectPtrLabel_t = unsafe extern "system" fn(
  ptr: *const c_void,
  bufSize: GLsizei,
  length: *mut GLsizei,
  label: *mut GLchar,
);
pub type glGetObjectPtrLabelKHR_t = unsafe extern "system" fn(
  ptr: *const c_void,
  bufSize: GLsizei,
  length: *mut GLsizei,
  label: *mut GLchar,
);
pub type glGetPointerv_t = unsafe extern "system" fn(pname: GLenum, params: *mut *mut c_void);
pub type glGetPointervKHR_t = unsafe extern "system" fn(pname: GLenum, params: *mut *mut c_void);
pub type glGetProgramBinary_t = unsafe extern "system" fn(
  program: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  binaryFormat: *mut GLenum,
  binary: *mut c_void,
);
pub type glGetProgramInfoLog_t = unsafe extern "system" fn(
  program: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  infoLog: *mut GLchar,
);
pub type glGetProgramInterfaceiv_t = unsafe extern "system" fn(
  program: GLuint,
  programInterface: GLenum,
  pname: GLenum,
  params: *mut GLint,
);
pub type glGetProgramPipelineInfoLog_t = unsafe extern "system" fn(
  pipeline: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  infoLog: *mut GLchar,
);
pub type glGetProgramPipelineiv_t =
  unsafe extern "system" fn(pipeline: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetProgramResourceIndex_t = unsafe extern "system" fn(
  program: GLuint,
  programInterface: GLenum,
  name: *const GLchar,
) -> GLuint;
pub type glGetProgramResourceLocation_t = unsafe extern "system" fn(
  program: GLuint,
  programInterface: GLenum,
  name: *const GLchar,
) -> GLint;
pub type glGetProgramResourceLocationIndex_t = unsafe extern "system" fn(
  program: GLuint,
  programInterface: GLenum,
  name: *const GLchar,
) -> GLint;
pub type glGetProgramResourceName_t = unsafe extern "system" fn(
  program: GLuint,
  programInterface: GLenum,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  name: *mut GLchar,
);
pub type glGetProgramResourceiv_t = unsafe extern "system" fn(
  program: GLuint,
  programInterface: GLenum,
  index: GLuint,
  propCount: GLsizei,
  props: *const GLenum,
  count: GLsizei,
  length: *mut GLsizei,
  params: *mut GLint,
);
pub type glGetProgramStageiv_t =
  unsafe extern "system" fn(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint);
pub type glGetProgramiv_t =
  unsafe extern "system" fn(program: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetQueryBufferObjecti64v_t =
  unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
pub type glGetQueryBufferObjectiv_t =
  unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
pub type glGetQueryBufferObjectui64v_t =
  unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
pub type glGetQueryBufferObjectuiv_t =
  unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
pub type glGetQueryIndexediv_t =
  unsafe extern "system" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetQueryObjecti64v_t =
  unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint64);
pub type glGetQueryObjectiv_t =
  unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetQueryObjectui64v_t =
  unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint64);
pub type glGetQueryObjectuiv_t =
  unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint);
pub type glGetQueryiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type glGetRenderbufferParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type glGetSamplerParameterIiv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetSamplerParameterIuiv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint);
pub type glGetSamplerParameterfv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
pub type glGetSamplerParameteriv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetShaderInfoLog_t = unsafe extern "system" fn(
  shader: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  infoLog: *mut GLchar,
);
pub type glGetShaderPrecisionFormat_t = unsafe extern "system" fn(
  shadertype: GLenum,
  precisiontype: GLenum,
  range: *mut GLint,
  precision: *mut GLint,
);
pub type glGetShaderSource_t = unsafe extern "system" fn(
  shader: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  source: *mut GLchar,
);
pub type glGetShaderiv_t =
  unsafe extern "system" fn(shader: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetString_t = unsafe extern "system" fn(name: GLenum) -> *const GLubyte;
pub type glGetStringi_t = unsafe extern "system" fn(name: GLenum, index: GLuint) -> *const GLubyte;
pub type glGetSubroutineIndex_t =
  unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint;
pub type glGetSubroutineUniformLocation_t =
  unsafe extern "system" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint;
pub type glGetSynciv_t = unsafe extern "system" fn(
  sync: GLsync,
  pname: GLenum,
  count: GLsizei,
  length: *mut GLsizei,
  values: *mut GLint,
);
pub type glGetTexImage_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  format: GLenum,
  ty: GLenum,
  pixels: *mut c_void,
);
pub type glGetTexLevelParameterfv_t =
  unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
pub type glGetTexLevelParameteriv_t =
  unsafe extern "system" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
pub type glGetTexParameterIiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type glGetTexParameterIuiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLuint);
pub type glGetTexParameterfv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);
pub type glGetTexParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type glGetTextureImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  pixels: *mut c_void,
);
pub type glGetTextureLevelParameterfv_t =
  unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat);
pub type glGetTextureLevelParameteriv_t =
  unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint);
pub type glGetTextureParameterIiv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetTextureParameterIuiv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLuint);
pub type glGetTextureParameterfv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLfloat);
pub type glGetTextureParameteriv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetTextureSubImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  pixels: *mut c_void,
);
pub type glGetTransformFeedbackVarying_t = unsafe extern "system" fn(
  program: GLuint,
  index: GLuint,
  bufSize: GLsizei,
  length: *mut GLsizei,
  size: *mut GLsizei,
  ty: *mut GLenum,
  name: *mut GLchar,
);
pub type glGetTransformFeedbacki64_v_t =
  unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64);
pub type glGetTransformFeedbacki_v_t =
  unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint);
pub type glGetTransformFeedbackiv_t =
  unsafe extern "system" fn(xfb: GLuint, pname: GLenum, param: *mut GLint);
pub type glGetUniformBlockIndex_t =
  unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
pub type glGetUniformIndices_t = unsafe extern "system" fn(
  program: GLuint,
  uniformCount: GLsizei,
  uniformNames: *const *const GLchar,
  uniformIndices: *mut GLuint,
);
pub type glGetUniformLocation_t =
  unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type glGetUniformSubroutineuiv_t =
  unsafe extern "system" fn(shadertype: GLenum, location: GLint, params: *mut GLuint);
pub type glGetUniformdv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLdouble);
pub type glGetUniformfv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat);
pub type glGetUniformiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint);
pub type glGetUniformuiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);
pub type glGetVertexArrayIndexed64iv_t =
  unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64);
pub type glGetVertexArrayIndexediv_t =
  unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);
pub type glGetVertexArrayiv_t =
  unsafe extern "system" fn(vaobj: GLuint, pname: GLenum, param: *mut GLint);
pub type glGetVertexAttribIiv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetVertexAttribIuiv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint);
pub type glGetVertexAttribLdv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
pub type glGetVertexAttribPointerv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
pub type glGetVertexAttribdv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
pub type glGetVertexAttribfv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLfloat);
pub type glGetVertexAttribiv_t =
  unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint);
pub type glGetnColorTable_t = unsafe extern "system" fn(
  target: GLenum,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  table: *mut c_void,
);
pub type glGetnCompressedTexImage_t =
  unsafe extern "system" fn(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void);
pub type glGetnConvolutionFilter_t = unsafe extern "system" fn(
  target: GLenum,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  image: *mut c_void,
);
pub type glGetnHistogram_t = unsafe extern "system" fn(
  target: GLenum,
  reset: GLboolean,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  values: *mut c_void,
);
pub type glGetnMapdv_t =
  unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble);
pub type glGetnMapfv_t =
  unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat);
pub type glGetnMapiv_t =
  unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint);
pub type glGetnMinmax_t = unsafe extern "system" fn(
  target: GLenum,
  reset: GLboolean,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  values: *mut c_void,
);
pub type glGetnPixelMapfv_t =
  unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLfloat);
pub type glGetnPixelMapuiv_t =
  unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLuint);
pub type glGetnPixelMapusv_t =
  unsafe extern "system" fn(map: GLenum, bufSize: GLsizei, values: *mut GLushort);
pub type glGetnPolygonStipple_t =
  unsafe extern "system" fn(bufSize: GLsizei, pattern: *mut GLubyte);
pub type glGetnSeparableFilter_t = unsafe extern "system" fn(
  target: GLenum,
  format: GLenum,
  ty: GLenum,
  rowBufSize: GLsizei,
  row: *mut c_void,
  columnBufSize: GLsizei,
  column: *mut c_void,
  span: *mut c_void,
);
pub type glGetnTexImage_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  pixels: *mut c_void,
);
pub type glGetnUniformdv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  bufSize: GLsizei,
  params: *mut GLdouble,
);
pub type glGetnUniformfv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  bufSize: GLsizei,
  params: *mut GLfloat,
);
pub type glGetnUniformiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);
pub type glGetnUniformuiv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  bufSize: GLsizei,
  params: *mut GLuint,
);
pub type glHint_t = unsafe extern "system" fn(target: GLenum, mode: GLenum);
pub type glInvalidateBufferData_t = unsafe extern "system" fn(buffer: GLuint);
pub type glInvalidateBufferSubData_t =
  unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
pub type glInvalidateFramebuffer_t =
  unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);
pub type glInvalidateNamedFramebufferData_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  numAttachments: GLsizei,
  attachments: *const GLenum,
);
pub type glInvalidateNamedFramebufferSubData_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  numAttachments: GLsizei,
  attachments: *const GLenum,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
pub type glInvalidateSubFramebuffer_t = unsafe extern "system" fn(
  target: GLenum,
  numAttachments: GLsizei,
  attachments: *const GLenum,
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
);
pub type glInvalidateTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint);
pub type glInvalidateTexSubImage_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
);
pub type glIsBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;
pub type glIsEnabled_t = unsafe extern "system" fn(cap: GLenum) -> GLboolean;
pub type glIsEnabledi_t = unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean;
pub type glIsFramebuffer_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;
pub type glIsProgram_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;
pub type glIsProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint) -> GLboolean;
pub type glIsQuery_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;
pub type glIsRenderbuffer_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;
pub type glIsSampler_t = unsafe extern "system" fn(sampler: GLuint) -> GLboolean;
pub type glIsShader_t = unsafe extern "system" fn(shader: GLuint) -> GLboolean;
pub type glIsSync_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;
pub type glIsTexture_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;
pub type glIsTransformFeedback_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;
pub type glIsVertexArray_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;
pub type glLineWidth_t = unsafe extern "system" fn(width: GLfloat);
pub type glLinkProgram_t = unsafe extern "system" fn(program: GLuint);
pub type glLogicOp_t = unsafe extern "system" fn(opcode: GLenum);
pub type glMapBuffer_t = unsafe extern "system" fn(target: GLenum, access: GLenum) -> *mut c_void;
pub type glMapBufferRange_t = unsafe extern "system" fn(
  target: GLenum,
  offset: GLintptr,
  length: GLsizeiptr,
  access: GLbitfield,
) -> *mut c_void;
pub type glMapNamedBuffer_t =
  unsafe extern "system" fn(buffer: GLuint, access: GLenum) -> *mut c_void;
pub type glMapNamedBufferRange_t = unsafe extern "system" fn(
  buffer: GLuint,
  offset: GLintptr,
  length: GLsizeiptr,
  access: GLbitfield,
) -> *mut c_void;
pub type glMemoryBarrier_t = unsafe extern "system" fn(barriers: GLbitfield);
pub type glMemoryBarrierByRegion_t = unsafe extern "system" fn(barriers: GLbitfield);
pub type glMinSampleShading_t = unsafe extern "system" fn(value: GLfloat);
pub type glMultiDrawArrays_t = unsafe extern "system" fn(
  mode: GLenum,
  first: *const GLint,
  count: *const GLsizei,
  drawcount: GLsizei,
);
pub type glMultiDrawArraysIndirect_t = unsafe extern "system" fn(
  mode: GLenum,
  indirect: *const c_void,
  drawcount: GLsizei,
  stride: GLsizei,
);
pub type glMultiDrawArraysIndirectCount_t = unsafe extern "system" fn(
  mode: GLenum,
  indirect: *const c_void,
  drawcount: GLintptr,
  maxdrawcount: GLsizei,
  stride: GLsizei,
);
pub type glMultiDrawElements_t = unsafe extern "system" fn(
  mode: GLenum,
  count: *const GLsizei,
  ty: GLenum,
  indices: *const *const c_void,
  drawcount: GLsizei,
);
pub type glMultiDrawElementsBaseVertex_t = unsafe extern "system" fn(
  mode: GLenum,
  count: *const GLsizei,
  ty: GLenum,
  indices: *const *const c_void,
  drawcount: GLsizei,
  basevertex: *const GLint,
);
pub type glMultiDrawElementsIndirect_t = unsafe extern "system" fn(
  mode: GLenum,
  ty: GLenum,
  indirect: *const c_void,
  drawcount: GLsizei,
  stride: GLsizei,
);
pub type glMultiDrawElementsIndirectCount_t = unsafe extern "system" fn(
  mode: GLenum,
  ty: GLenum,
  indirect: *const c_void,
  drawcount: GLintptr,
  maxdrawcount: GLsizei,
  stride: GLsizei,
);
pub type glMultiTexCoordP1ui_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
pub type glMultiTexCoordP1uiv_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
pub type glMultiTexCoordP2ui_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
pub type glMultiTexCoordP2uiv_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
pub type glMultiTexCoordP3ui_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
pub type glMultiTexCoordP3uiv_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
pub type glMultiTexCoordP4ui_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: GLuint);
pub type glMultiTexCoordP4uiv_t =
  unsafe extern "system" fn(texture: GLenum, ty: GLenum, coords: *const GLuint);
pub type glNamedBufferData_t =
  unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum);
pub type glNamedBufferStorage_t = unsafe extern "system" fn(
  buffer: GLuint,
  size: GLsizeiptr,
  data: *const c_void,
  flags: GLbitfield,
);
pub type glNamedBufferSubData_t = unsafe extern "system" fn(
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
  data: *const c_void,
);
pub type glNamedFramebufferDrawBuffer_t =
  unsafe extern "system" fn(framebuffer: GLuint, buf: GLenum);
pub type glNamedFramebufferDrawBuffers_t =
  unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);
pub type glNamedFramebufferParameteri_t =
  unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: GLint);
pub type glNamedFramebufferReadBuffer_t =
  unsafe extern "system" fn(framebuffer: GLuint, src: GLenum);
pub type glNamedFramebufferRenderbuffer_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  attachment: GLenum,
  renderbuffertarget: GLenum,
  renderbuffer: GLuint,
);
pub type glNamedFramebufferTexture_t =
  unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);
pub type glNamedFramebufferTextureLayer_t = unsafe extern "system" fn(
  framebuffer: GLuint,
  attachment: GLenum,
  texture: GLuint,
  level: GLint,
  layer: GLint,
);
pub type glNamedRenderbufferStorage_t = unsafe extern "system" fn(
  renderbuffer: GLuint,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
pub type glNamedRenderbufferStorageMultisample_t = unsafe extern "system" fn(
  renderbuffer: GLuint,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
pub type glNormalP3ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
pub type glNormalP3uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
pub type glObjectLabel_t = unsafe extern "system" fn(
  identifier: GLenum,
  name: GLuint,
  length: GLsizei,
  label: *const GLchar,
);
pub type glObjectLabelKHR_t = unsafe extern "system" fn(
  identifier: GLenum,
  name: GLuint,
  length: GLsizei,
  label: *const GLchar,
);
pub type glObjectPtrLabel_t =
  unsafe extern "system" fn(ptr: *const c_void, length: GLsizei, label: *const GLchar);
pub type glObjectPtrLabelKHR_t =
  unsafe extern "system" fn(ptr: *const c_void, length: GLsizei, label: *const GLchar);
pub type glPatchParameterfv_t = unsafe extern "system" fn(pname: GLenum, values: *const GLfloat);
pub type glPatchParameteri_t = unsafe extern "system" fn(pname: GLenum, value: GLint);
pub type glPauseTransformFeedback_t = unsafe extern "system" fn();
pub type glPixelStoref_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);
pub type glPixelStorei_t = unsafe extern "system" fn(pname: GLenum, param: GLint);
pub type glPointParameterf_t = unsafe extern "system" fn(pname: GLenum, param: GLfloat);
pub type glPointParameterfv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLfloat);
pub type glPointParameteri_t = unsafe extern "system" fn(pname: GLenum, param: GLint);
pub type glPointParameteriv_t = unsafe extern "system" fn(pname: GLenum, params: *const GLint);
pub type glPointSize_t = unsafe extern "system" fn(size: GLfloat);
pub type glPolygonMode_t = unsafe extern "system" fn(face: GLenum, mode: GLenum);
pub type glPolygonOffset_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);
pub type glPolygonOffsetClamp_t =
  unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);
pub type glPopDebugGroup_t = unsafe extern "system" fn();
pub type glPopDebugGroupKHR_t = unsafe extern "system" fn();
pub type glPrimitiveBoundingBox_t = unsafe extern "system" fn(
  minX: GLfloat,
  minY: GLfloat,
  minZ: GLfloat,
  minW: GLfloat,
  maxX: GLfloat,
  maxY: GLfloat,
  maxZ: GLfloat,
  maxW: GLfloat,
);
pub type glPrimitiveRestartIndex_t = unsafe extern "system" fn(index: GLuint);
pub type glProgramBinary_t = unsafe extern "system" fn(
  program: GLuint,
  binaryFormat: GLenum,
  binary: *const c_void,
  length: GLsizei,
);
pub type glProgramParameteri_t =
  unsafe extern "system" fn(program: GLuint, pname: GLenum, value: GLint);
pub type glProgramUniform1d_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble);
pub type glProgramUniform1dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLdouble,
);
pub type glProgramUniform1f_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);
pub type glProgramUniform1fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLfloat,
);
pub type glProgramUniform1i_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);
pub type glProgramUniform1iv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
pub type glProgramUniform1ui_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);
pub type glProgramUniform1uiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
pub type glProgramUniform2d_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);
pub type glProgramUniform2dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLdouble,
);
pub type glProgramUniform2f_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
pub type glProgramUniform2fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLfloat,
);
pub type glProgramUniform2i_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);
pub type glProgramUniform2iv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
pub type glProgramUniform2ui_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
pub type glProgramUniform2uiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
pub type glProgramUniform3d_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLdouble,
  v1: GLdouble,
  v2: GLdouble,
);
pub type glProgramUniform3dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLdouble,
);
pub type glProgramUniform3f_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLfloat,
  v1: GLfloat,
  v2: GLfloat,
);
pub type glProgramUniform3fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLfloat,
);
pub type glProgramUniform3i_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
pub type glProgramUniform3iv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
pub type glProgramUniform3ui_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
pub type glProgramUniform3uiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
pub type glProgramUniform4d_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLdouble,
  v1: GLdouble,
  v2: GLdouble,
  v3: GLdouble,
);
pub type glProgramUniform4dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLdouble,
);
pub type glProgramUniform4f_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLfloat,
  v1: GLfloat,
  v2: GLfloat,
  v3: GLfloat,
);
pub type glProgramUniform4fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  value: *const GLfloat,
);
pub type glProgramUniform4i_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLint,
  v1: GLint,
  v2: GLint,
  v3: GLint,
);
pub type glProgramUniform4iv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
pub type glProgramUniform4ui_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  v0: GLuint,
  v1: GLuint,
  v2: GLuint,
  v3: GLuint,
);
pub type glProgramUniform4uiv_t =
  unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
pub type glProgramUniformMatrix2dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glProgramUniformMatrix2fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glProgramUniformMatrix2x3dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glProgramUniformMatrix2x3fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glProgramUniformMatrix2x4dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glProgramUniformMatrix2x4fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glProgramUniformMatrix3dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glProgramUniformMatrix3fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glProgramUniformMatrix3x2dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glProgramUniformMatrix3x2fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glProgramUniformMatrix3x4dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glProgramUniformMatrix3x4fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glProgramUniformMatrix4dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glProgramUniformMatrix4fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glProgramUniformMatrix4x2dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glProgramUniformMatrix4x2fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glProgramUniformMatrix4x3dv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glProgramUniformMatrix4x3fv_t = unsafe extern "system" fn(
  program: GLuint,
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glProvokingVertex_t = unsafe extern "system" fn(mode: GLenum);
pub type glPushDebugGroup_t =
  unsafe extern "system" fn(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);
pub type glPushDebugGroupKHR_t =
  unsafe extern "system" fn(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);
pub type glQueryCounter_t = unsafe extern "system" fn(id: GLuint, target: GLenum);
pub type glReadBuffer_t = unsafe extern "system" fn(src: GLenum);
pub type glReadPixels_t = unsafe extern "system" fn(
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *mut c_void,
);
pub type glReadnPixels_t = unsafe extern "system" fn(
  x: GLint,
  y: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  ty: GLenum,
  bufSize: GLsizei,
  data: *mut c_void,
);
pub type glReleaseShaderCompiler_t = unsafe extern "system" fn();
pub type glRenderbufferStorage_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
pub type glRenderbufferStorageMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
pub type glResumeTransformFeedback_t = unsafe extern "system" fn();
pub type glSampleCoverage_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);
pub type glSampleMaski_t = unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield);
pub type glSamplerParameterIiv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
pub type glSamplerParameterIuiv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLuint);
pub type glSamplerParameterf_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLfloat);
pub type glSamplerParameterfv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLfloat);
pub type glSamplerParameteri_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLint);
pub type glSamplerParameteriv_t =
  unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
pub type glScissor_t =
  unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type glScissorArrayv_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);
pub type glScissorIndexed_t = unsafe extern "system" fn(
  index: GLuint,
  left: GLint,
  bottom: GLint,
  width: GLsizei,
  height: GLsizei,
);
pub type glScissorIndexedv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type glSecondaryColorP3ui_t = unsafe extern "system" fn(ty: GLenum, color: GLuint);
pub type glSecondaryColorP3uiv_t = unsafe extern "system" fn(ty: GLenum, color: *const GLuint);
pub type glShaderBinary_t = unsafe extern "system" fn(
  count: GLsizei,
  shaders: *const GLuint,
  binaryFormat: GLenum,
  binary: *const c_void,
  length: GLsizei,
);
pub type glShaderSource_t = unsafe extern "system" fn(
  shader: GLuint,
  count: GLsizei,
  string: *const *const GLchar,
  length: *const GLint,
);
pub type glShaderStorageBlockBinding_t = unsafe extern "system" fn(
  program: GLuint,
  storageBlockIndex: GLuint,
  storageBlockBinding: GLuint,
);
pub type glSpecializeShader_t = unsafe extern "system" fn(
  shader: GLuint,
  pEntryPoint: *const GLchar,
  numSpecializationConstants: GLuint,
  pConstantIndex: *const GLuint,
  pConstantValue: *const GLuint,
);
pub type glStencilFunc_t = unsafe extern "system" fn(func: GLenum, reference: GLint, mask: GLuint);
pub type glStencilFuncSeparate_t =
  unsafe extern "system" fn(face: GLenum, func: GLenum, reference: GLint, mask: GLuint);
pub type glStencilMask_t = unsafe extern "system" fn(mask: GLuint);
pub type glStencilMaskSeparate_t = unsafe extern "system" fn(face: GLenum, mask: GLuint);
pub type glStencilOp_t = unsafe extern "system" fn(fail: GLenum, zfail: GLenum, zpass: GLenum);
pub type glStencilOpSeparate_t =
  unsafe extern "system" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
pub type glTexBuffer_t =
  unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint);
pub type glTexBufferRange_t = unsafe extern "system" fn(
  target: GLenum,
  internalformat: GLenum,
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
);
pub type glTexCoordP1ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
pub type glTexCoordP1uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
pub type glTexCoordP2ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
pub type glTexCoordP2uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
pub type glTexCoordP3ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
pub type glTexCoordP3uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
pub type glTexCoordP4ui_t = unsafe extern "system" fn(ty: GLenum, coords: GLuint);
pub type glTexCoordP4uiv_t = unsafe extern "system" fn(ty: GLenum, coords: *const GLuint);
pub type glTexImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLint,
  width: GLsizei,
  border: GLint,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
pub type glTexImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLint,
  width: GLsizei,
  height: GLsizei,
  border: GLint,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
pub type glTexImage2DMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  fixedsamplelocations: GLboolean,
);
pub type glTexImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  internalformat: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  border: GLint,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
pub type glTexImage3DMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  fixedsamplelocations: GLboolean,
);
pub type glTexParameterIiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);
pub type glTexParameterIuiv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLuint);
pub type glTexParameterf_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLfloat);
pub type glTexParameterfv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLfloat);
pub type glTexParameteri_t = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);
pub type glTexParameteriv_t =
  unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *const GLint);
pub type glTexStorage1D_t = unsafe extern "system" fn(
  target: GLenum,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
);
pub type glTexStorage2D_t = unsafe extern "system" fn(
  target: GLenum,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
pub type glTexStorage2DMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  fixedsamplelocations: GLboolean,
);
pub type glTexStorage3D_t = unsafe extern "system" fn(
  target: GLenum,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
);
pub type glTexStorage3DMultisample_t = unsafe extern "system" fn(
  target: GLenum,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  fixedsamplelocations: GLboolean,
);
pub type glTexSubImage1D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  width: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
pub type glTexSubImage2D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
pub type glTexSubImage3D_t = unsafe extern "system" fn(
  target: GLenum,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
pub type glTextureBarrier_t = unsafe extern "system" fn();
pub type glTextureBuffer_t =
  unsafe extern "system" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint);
pub type glTextureBufferRange_t = unsafe extern "system" fn(
  texture: GLuint,
  internalformat: GLenum,
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
);
pub type glTextureParameterIiv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLint);
pub type glTextureParameterIuiv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLuint);
pub type glTextureParameterf_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLfloat);
pub type glTextureParameterfv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLfloat);
pub type glTextureParameteri_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLint);
pub type glTextureParameteriv_t =
  unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLint);
pub type glTextureStorage1D_t = unsafe extern "system" fn(
  texture: GLuint,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
);
pub type glTextureStorage2D_t = unsafe extern "system" fn(
  texture: GLuint,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
);
pub type glTextureStorage2DMultisample_t = unsafe extern "system" fn(
  texture: GLuint,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  fixedsamplelocations: GLboolean,
);
pub type glTextureStorage3D_t = unsafe extern "system" fn(
  texture: GLuint,
  levels: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
);
pub type glTextureStorage3DMultisample_t = unsafe extern "system" fn(
  texture: GLuint,
  samples: GLsizei,
  internalformat: GLenum,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  fixedsamplelocations: GLboolean,
);
pub type glTextureSubImage1D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  width: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
pub type glTextureSubImage2D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
pub type glTextureSubImage3D_t = unsafe extern "system" fn(
  texture: GLuint,
  level: GLint,
  xoffset: GLint,
  yoffset: GLint,
  zoffset: GLint,
  width: GLsizei,
  height: GLsizei,
  depth: GLsizei,
  format: GLenum,
  ty: GLenum,
  pixels: *const c_void,
);
pub type glTextureView_t = unsafe extern "system" fn(
  texture: GLuint,
  target: GLenum,
  origtexture: GLuint,
  internalformat: GLenum,
  minlevel: GLuint,
  numlevels: GLuint,
  minlayer: GLuint,
  numlayers: GLuint,
);
pub type glTransformFeedbackBufferBase_t =
  unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint);
pub type glTransformFeedbackBufferRange_t = unsafe extern "system" fn(
  xfb: GLuint,
  index: GLuint,
  buffer: GLuint,
  offset: GLintptr,
  size: GLsizeiptr,
);
pub type glTransformFeedbackVaryings_t = unsafe extern "system" fn(
  program: GLuint,
  count: GLsizei,
  varyings: *const *const GLchar,
  bufferMode: GLenum,
);
pub type glUniform1d_t = unsafe extern "system" fn(location: GLint, x: GLdouble);
pub type glUniform1dv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
pub type glUniform1f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);
pub type glUniform1fv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type glUniform1i_t = unsafe extern "system" fn(location: GLint, v0: GLint);
pub type glUniform1iv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type glUniform1ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint);
pub type glUniform1uiv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type glUniform2d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble);
pub type glUniform2dv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
pub type glUniform2f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);
pub type glUniform2fv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type glUniform2i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);
pub type glUniform2iv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type glUniform2ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);
pub type glUniform2uiv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type glUniform3d_t =
  unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
pub type glUniform3dv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
pub type glUniform3f_t =
  unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
pub type glUniform3fv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type glUniform3i_t =
  unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);
pub type glUniform3iv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type glUniform3ui_t =
  unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
pub type glUniform3uiv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type glUniform4d_t =
  unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type glUniform4dv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);
pub type glUniform4f_t =
  unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
pub type glUniform4fv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type glUniform4i_t =
  unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
pub type glUniform4iv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type glUniform4ui_t =
  unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
pub type glUniform4uiv_t =
  unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type glUniformBlockBinding_t = unsafe extern "system" fn(
  program: GLuint,
  uniformBlockIndex: GLuint,
  uniformBlockBinding: GLuint,
);
pub type glUniformMatrix2dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glUniformMatrix2fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glUniformMatrix2x3dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glUniformMatrix2x3fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glUniformMatrix2x4dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glUniformMatrix2x4fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glUniformMatrix3dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glUniformMatrix3fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glUniformMatrix3x2dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glUniformMatrix3x2fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glUniformMatrix3x4dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glUniformMatrix3x4fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glUniformMatrix4dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glUniformMatrix4fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glUniformMatrix4x2dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glUniformMatrix4x2fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glUniformMatrix4x3dv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLdouble,
);
pub type glUniformMatrix4x3fv_t = unsafe extern "system" fn(
  location: GLint,
  count: GLsizei,
  transpose: GLboolean,
  value: *const GLfloat,
);
pub type glUniformSubroutinesuiv_t =
  unsafe extern "system" fn(shadertype: GLenum, count: GLsizei, indices: *const GLuint);
pub type glUnmapBuffer_t = unsafe extern "system" fn(target: GLenum) -> GLboolean;
pub type glUnmapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;
pub type glUseProgram_t = unsafe extern "system" fn(program: GLuint);
pub type glUseProgramStages_t =
  unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);
pub type glValidateProgram_t = unsafe extern "system" fn(program: GLuint);
pub type glValidateProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);
pub type glVertexArrayAttribBinding_t =
  unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
pub type glVertexArrayAttribFormat_t = unsafe extern "system" fn(
  vaobj: GLuint,
  attribindex: GLuint,
  size: GLint,
  ty: GLenum,
  normalized: GLboolean,
  relativeoffset: GLuint,
);
pub type glVertexArrayAttribIFormat_t = unsafe extern "system" fn(
  vaobj: GLuint,
  attribindex: GLuint,
  size: GLint,
  ty: GLenum,
  relativeoffset: GLuint,
);
pub type glVertexArrayAttribLFormat_t = unsafe extern "system" fn(
  vaobj: GLuint,
  attribindex: GLuint,
  size: GLint,
  ty: GLenum,
  relativeoffset: GLuint,
);
pub type glVertexArrayBindingDivisor_t =
  unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
pub type glVertexArrayElementBuffer_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint);
pub type glVertexArrayVertexBuffer_t = unsafe extern "system" fn(
  vaobj: GLuint,
  bindingindex: GLuint,
  buffer: GLuint,
  offset: GLintptr,
  stride: GLsizei,
);
pub type glVertexArrayVertexBuffers_t = unsafe extern "system" fn(
  vaobj: GLuint,
  first: GLuint,
  count: GLsizei,
  buffers: *const GLuint,
  offsets: *const GLintptr,
  strides: *const GLsizei,
);
pub type glVertexAttrib1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);
pub type glVertexAttrib1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type glVertexAttrib1f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);
pub type glVertexAttrib1fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
pub type glVertexAttrib1s_t = unsafe extern "system" fn(index: GLuint, x: GLshort);
pub type glVertexAttrib1sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type glVertexAttrib2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);
pub type glVertexAttrib2dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type glVertexAttrib2f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);
pub type glVertexAttrib2fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
pub type glVertexAttrib2s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);
pub type glVertexAttrib2sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type glVertexAttrib3d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
pub type glVertexAttrib3dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type glVertexAttrib3f_t =
  unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
pub type glVertexAttrib3fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
pub type glVertexAttrib3s_t =
  unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);
pub type glVertexAttrib3sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type glVertexAttrib4Nbv_t = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
pub type glVertexAttrib4Niv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type glVertexAttrib4Nsv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type glVertexAttrib4Nub_t =
  unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
pub type glVertexAttrib4Nubv_t = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
pub type glVertexAttrib4Nuiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type glVertexAttrib4Nusv_t = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
pub type glVertexAttrib4bv_t = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
pub type glVertexAttrib4d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type glVertexAttrib4dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type glVertexAttrib4f_t =
  unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
pub type glVertexAttrib4fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
pub type glVertexAttrib4iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type glVertexAttrib4s_t =
  unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
pub type glVertexAttrib4sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type glVertexAttrib4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
pub type glVertexAttrib4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type glVertexAttrib4usv_t = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
pub type glVertexAttribBinding_t =
  unsafe extern "system" fn(attribindex: GLuint, bindingindex: GLuint);
pub type glVertexAttribDivisor_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);
pub type glVertexAttribFormat_t = unsafe extern "system" fn(
  attribindex: GLuint,
  size: GLint,
  ty: GLenum,
  normalized: GLboolean,
  relativeoffset: GLuint,
);
pub type glVertexAttribI1i_t = unsafe extern "system" fn(index: GLuint, x: GLint);
pub type glVertexAttribI1iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type glVertexAttribI1ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint);
pub type glVertexAttribI1uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type glVertexAttribI2i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);
pub type glVertexAttribI2iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type glVertexAttribI2ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);
pub type glVertexAttribI2uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type glVertexAttribI3i_t =
  unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);
pub type glVertexAttribI3iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type glVertexAttribI3ui_t =
  unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);
pub type glVertexAttribI3uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type glVertexAttribI4bv_t = unsafe extern "system" fn(index: GLuint, v: *const GLbyte);
pub type glVertexAttribI4i_t =
  unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
pub type glVertexAttribI4iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);
pub type glVertexAttribI4sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);
pub type glVertexAttribI4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const GLubyte);
pub type glVertexAttribI4ui_t =
  unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
pub type glVertexAttribI4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);
pub type glVertexAttribI4usv_t = unsafe extern "system" fn(index: GLuint, v: *const GLushort);
pub type glVertexAttribIFormat_t =
  unsafe extern "system" fn(attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint);
pub type glVertexAttribIPointer_t = unsafe extern "system" fn(
  index: GLuint,
  size: GLint,
  ty: GLenum,
  stride: GLsizei,
  pointer: *const c_void,
);
pub type glVertexAttribL1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);
pub type glVertexAttribL1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type glVertexAttribL2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);
pub type glVertexAttribL2dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type glVertexAttribL3d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
pub type glVertexAttribL3dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type glVertexAttribL4d_t =
  unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type glVertexAttribL4dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);
pub type glVertexAttribLFormat_t =
  unsafe extern "system" fn(attribindex: GLuint, size: GLint, ty: GLenum, relativeoffset: GLuint);
pub type glVertexAttribLPointer_t = unsafe extern "system" fn(
  index: GLuint,
  size: GLint,
  ty: GLenum,
  stride: GLsizei,
  pointer: *const c_void,
);
pub type glVertexAttribP1ui_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
pub type glVertexAttribP1uiv_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint);
pub type glVertexAttribP2ui_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
pub type glVertexAttribP2uiv_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint);
pub type glVertexAttribP3ui_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
pub type glVertexAttribP3uiv_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint);
pub type glVertexAttribP4ui_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: GLuint);
pub type glVertexAttribP4uiv_t =
  unsafe extern "system" fn(index: GLuint, ty: GLenum, normalized: GLboolean, value: *const GLuint);
pub type glVertexAttribPointer_t = unsafe extern "system" fn(
  index: GLuint,
  size: GLint,
  ty: GLenum,
  normalized: GLboolean,
  stride: GLsizei,
  pointer: *const c_void,
);
pub type glVertexBindingDivisor_t =
  unsafe extern "system" fn(bindingindex: GLuint, divisor: GLuint);
pub type glVertexP2ui_t = unsafe extern "system" fn(ty: GLenum, value: GLuint);
pub type glVertexP2uiv_t = unsafe extern "system" fn(ty: GLenum, value: *const GLuint);
pub type glVertexP3ui_t = unsafe extern "system" fn(ty: GLenum, value: GLuint);
pub type glVertexP3uiv_t = unsafe extern "system" fn(ty: GLenum, value: *const GLuint);
pub type glVertexP4ui_t = unsafe extern "system" fn(ty: GLenum, value: GLuint);
pub type glVertexP4uiv_t = unsafe extern "system" fn(ty: GLenum, value: *const GLuint);
pub type glViewport_t =
  unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type glViewportArrayv_t =
  unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);
pub type glViewportIndexedf_t =
  unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);
pub type glViewportIndexedfv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);
pub type glWaitSync_t =
  unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
