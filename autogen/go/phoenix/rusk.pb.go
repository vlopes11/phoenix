// Code generated by protoc-gen-go. DO NOT EDIT.
// source: rusk.proto

package phoenix

import (
	context "context"
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	math "math"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

type EchoRequest struct {
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *EchoRequest) Reset()         { *m = EchoRequest{} }
func (m *EchoRequest) String() string { return proto.CompactTextString(m) }
func (*EchoRequest) ProtoMessage()    {}
func (*EchoRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_211ec4e18343b18f, []int{0}
}

func (m *EchoRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_EchoRequest.Unmarshal(m, b)
}
func (m *EchoRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_EchoRequest.Marshal(b, m, deterministic)
}
func (m *EchoRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_EchoRequest.Merge(m, src)
}
func (m *EchoRequest) XXX_Size() int {
	return xxx_messageInfo_EchoRequest.Size(m)
}
func (m *EchoRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_EchoRequest.DiscardUnknown(m)
}

var xxx_messageInfo_EchoRequest proto.InternalMessageInfo

type EchoResponse struct {
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *EchoResponse) Reset()         { *m = EchoResponse{} }
func (m *EchoResponse) String() string { return proto.CompactTextString(m) }
func (*EchoResponse) ProtoMessage()    {}
func (*EchoResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_211ec4e18343b18f, []int{1}
}

func (m *EchoResponse) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_EchoResponse.Unmarshal(m, b)
}
func (m *EchoResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_EchoResponse.Marshal(b, m, deterministic)
}
func (m *EchoResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_EchoResponse.Merge(m, src)
}
func (m *EchoResponse) XXX_Size() int {
	return xxx_messageInfo_EchoResponse.Size(m)
}
func (m *EchoResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_EchoResponse.DiscardUnknown(m)
}

var xxx_messageInfo_EchoResponse proto.InternalMessageInfo

type ValidateStateTransitionRequest struct {
	// List of transactions to be validated
	Txs                  []*Transaction `protobuf:"bytes,1,rep,name=txs,proto3" json:"txs,omitempty"`
	XXX_NoUnkeyedLiteral struct{}       `json:"-"`
	XXX_unrecognized     []byte         `json:"-"`
	XXX_sizecache        int32          `json:"-"`
}

func (m *ValidateStateTransitionRequest) Reset()         { *m = ValidateStateTransitionRequest{} }
func (m *ValidateStateTransitionRequest) String() string { return proto.CompactTextString(m) }
func (*ValidateStateTransitionRequest) ProtoMessage()    {}
func (*ValidateStateTransitionRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_211ec4e18343b18f, []int{2}
}

func (m *ValidateStateTransitionRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_ValidateStateTransitionRequest.Unmarshal(m, b)
}
func (m *ValidateStateTransitionRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_ValidateStateTransitionRequest.Marshal(b, m, deterministic)
}
func (m *ValidateStateTransitionRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ValidateStateTransitionRequest.Merge(m, src)
}
func (m *ValidateStateTransitionRequest) XXX_Size() int {
	return xxx_messageInfo_ValidateStateTransitionRequest.Size(m)
}
func (m *ValidateStateTransitionRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_ValidateStateTransitionRequest.DiscardUnknown(m)
}

var xxx_messageInfo_ValidateStateTransitionRequest proto.InternalMessageInfo

func (m *ValidateStateTransitionRequest) GetTxs() []*Transaction {
	if m != nil {
		return m.Txs
	}
	return nil
}

type ValidateStateTransitionResponse struct {
	// Status of the state transition
	Success              bool     `protobuf:"varint,1,opt,name=success,proto3" json:"success,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *ValidateStateTransitionResponse) Reset()         { *m = ValidateStateTransitionResponse{} }
func (m *ValidateStateTransitionResponse) String() string { return proto.CompactTextString(m) }
func (*ValidateStateTransitionResponse) ProtoMessage()    {}
func (*ValidateStateTransitionResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_211ec4e18343b18f, []int{3}
}

func (m *ValidateStateTransitionResponse) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_ValidateStateTransitionResponse.Unmarshal(m, b)
}
func (m *ValidateStateTransitionResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_ValidateStateTransitionResponse.Marshal(b, m, deterministic)
}
func (m *ValidateStateTransitionResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ValidateStateTransitionResponse.Merge(m, src)
}
func (m *ValidateStateTransitionResponse) XXX_Size() int {
	return xxx_messageInfo_ValidateStateTransitionResponse.Size(m)
}
func (m *ValidateStateTransitionResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_ValidateStateTransitionResponse.DiscardUnknown(m)
}

var xxx_messageInfo_ValidateStateTransitionResponse proto.InternalMessageInfo

func (m *ValidateStateTransitionResponse) GetSuccess() bool {
	if m != nil {
		return m.Success
	}
	return false
}

func init() {
	proto.RegisterType((*EchoRequest)(nil), "phoenix.EchoRequest")
	proto.RegisterType((*EchoResponse)(nil), "phoenix.EchoResponse")
	proto.RegisterType((*ValidateStateTransitionRequest)(nil), "phoenix.ValidateStateTransitionRequest")
	proto.RegisterType((*ValidateStateTransitionResponse)(nil), "phoenix.ValidateStateTransitionResponse")
}

func init() {
	proto.RegisterFile("rusk.proto", fileDescriptor_211ec4e18343b18f)
}

var fileDescriptor_211ec4e18343b18f = []byte{
	// 224 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0xe2, 0x2a, 0x2a, 0x2d, 0xce,
	0xd6, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0x62, 0x2f, 0xc8, 0xc8, 0x4f, 0xcd, 0xcb, 0xac, 0x90,
	0x12, 0x2c, 0x29, 0x4a, 0xcc, 0x2b, 0x4e, 0x4c, 0x2e, 0xc9, 0xcc, 0xcf, 0x83, 0xc8, 0x49, 0x71,
	0x65, 0xa7, 0x56, 0x16, 0x43, 0xd8, 0x4a, 0xbc, 0x5c, 0xdc, 0xae, 0xc9, 0x19, 0xf9, 0x41, 0xa9,
	0x85, 0xa5, 0xa9, 0xc5, 0x25, 0x4a, 0x7c, 0x5c, 0x3c, 0x10, 0x6e, 0x71, 0x41, 0x7e, 0x5e, 0x71,
	0xaa, 0x92, 0x07, 0x97, 0x5c, 0x58, 0x62, 0x4e, 0x66, 0x4a, 0x62, 0x49, 0x6a, 0x70, 0x49, 0x62,
	0x49, 0x6a, 0x08, 0xc8, 0xb0, 0x4c, 0x90, 0x59, 0x50, 0x1d, 0x42, 0x6a, 0x5c, 0xcc, 0x25, 0x15,
	0xc5, 0x12, 0x8c, 0x0a, 0xcc, 0x1a, 0xdc, 0x46, 0x22, 0x7a, 0x50, 0x6b, 0xf5, 0x42, 0x10, 0xb6,
	0x06, 0x81, 0x14, 0x28, 0x59, 0x73, 0xc9, 0xe3, 0x34, 0x09, 0x62, 0x99, 0x90, 0x04, 0x17, 0x7b,
	0x71, 0x69, 0x72, 0x72, 0x6a, 0x31, 0xc8, 0x38, 0x46, 0x0d, 0x8e, 0x20, 0x18, 0xd7, 0x68, 0x2d,
	0x23, 0x17, 0x4b, 0x50, 0x69, 0x71, 0xb6, 0x90, 0x29, 0x17, 0x0b, 0xc8, 0x7d, 0x42, 0x08, 0x8b,
	0x90, 0x5c, 0x2f, 0x25, 0x8a, 0x26, 0x0a, 0xf5, 0x04, 0x83, 0x50, 0x1e, 0x97, 0x38, 0x0e, 0xcb,
	0x85, 0xd4, 0xe1, 0x7a, 0xf0, 0x7b, 0x54, 0x4a, 0x83, 0xb0, 0x42, 0x98, 0x7d, 0x49, 0x6c, 0xe0,
	0xc0, 0x35, 0x06, 0x04, 0x00, 0x00, 0xff, 0xff, 0x49, 0x01, 0x3c, 0x2a, 0x92, 0x01, 0x00, 0x00,
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConnInterface

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion6

// RuskClient is the client API for Rusk service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type RuskClient interface {
	// Simple echo request
	Echo(ctx context.Context, in *EchoRequest, opts ...grpc.CallOption) (*EchoResponse, error)
	// Validate a set of transactions, returning false if at least one of the
	// listed transactions is inconsistent
	ValidateStateTransition(ctx context.Context, in *ValidateStateTransitionRequest, opts ...grpc.CallOption) (*ValidateStateTransitionResponse, error)
}

type ruskClient struct {
	cc grpc.ClientConnInterface
}

func NewRuskClient(cc grpc.ClientConnInterface) RuskClient {
	return &ruskClient{cc}
}

func (c *ruskClient) Echo(ctx context.Context, in *EchoRequest, opts ...grpc.CallOption) (*EchoResponse, error) {
	out := new(EchoResponse)
	err := c.cc.Invoke(ctx, "/phoenix.Rusk/Echo", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *ruskClient) ValidateStateTransition(ctx context.Context, in *ValidateStateTransitionRequest, opts ...grpc.CallOption) (*ValidateStateTransitionResponse, error) {
	out := new(ValidateStateTransitionResponse)
	err := c.cc.Invoke(ctx, "/phoenix.Rusk/ValidateStateTransition", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// RuskServer is the server API for Rusk service.
type RuskServer interface {
	// Simple echo request
	Echo(context.Context, *EchoRequest) (*EchoResponse, error)
	// Validate a set of transactions, returning false if at least one of the
	// listed transactions is inconsistent
	ValidateStateTransition(context.Context, *ValidateStateTransitionRequest) (*ValidateStateTransitionResponse, error)
}

// UnimplementedRuskServer can be embedded to have forward compatible implementations.
type UnimplementedRuskServer struct {
}

func (*UnimplementedRuskServer) Echo(ctx context.Context, req *EchoRequest) (*EchoResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Echo not implemented")
}
func (*UnimplementedRuskServer) ValidateStateTransition(ctx context.Context, req *ValidateStateTransitionRequest) (*ValidateStateTransitionResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ValidateStateTransition not implemented")
}

func RegisterRuskServer(s *grpc.Server, srv RuskServer) {
	s.RegisterService(&_Rusk_serviceDesc, srv)
}

func _Rusk_Echo_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(EchoRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(RuskServer).Echo(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/phoenix.Rusk/Echo",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(RuskServer).Echo(ctx, req.(*EchoRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Rusk_ValidateStateTransition_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ValidateStateTransitionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(RuskServer).ValidateStateTransition(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/phoenix.Rusk/ValidateStateTransition",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(RuskServer).ValidateStateTransition(ctx, req.(*ValidateStateTransitionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var _Rusk_serviceDesc = grpc.ServiceDesc{
	ServiceName: "phoenix.Rusk",
	HandlerType: (*RuskServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Echo",
			Handler:    _Rusk_Echo_Handler,
		},
		{
			MethodName: "ValidateStateTransition",
			Handler:    _Rusk_ValidateStateTransition_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "rusk.proto",
}
