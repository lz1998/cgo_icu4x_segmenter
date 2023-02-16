package cgo_icu4x_segmenter

/*
#cgo CFLAGS: -I./segmenter/target/release/
#cgo LDFLAGS: -L./segmenter/target/release -lsegmenter

#include "segmenter.h"
*/
import "C"

type SegmentResult struct {
	p *C.CharBreakPoints
}

func (r SegmentResult) Close() {
	C.free_char_break_points(r.p)
}

func (r SegmentResult) Next() int32 {
	return int32(C.next_break(r.p))
}

func cBreakSentence(s string) *SegmentResult {
	return &SegmentResult{p: C.break_sentence(C.CString(s))}
}

func cBreakWOrd(s string) *SegmentResult {
	return &SegmentResult{p: C.break_word(C.CString(s))}
}
