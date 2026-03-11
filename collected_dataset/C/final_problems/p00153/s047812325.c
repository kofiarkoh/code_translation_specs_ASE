#include<stdio.h>
#include<math.h>
//üª(x1,y1)-(x2,y2)Æ~(S(0,0),¼ar)ª¤L_ðÂ©»è
int has_common_circle_segment(int x1,int y1,int x2,int y2,int r){
	int x12=x2-x1;
	int y12=y2-y1;
	if(fabs(-y12*x1+x12*y1)/hypot(x12,y12)<=r){
		if(x2*x12+y2*y12>0&&-x1*x12-y1*y12>0){
			return 1;
		}
	}
	return 0;
}
//´_(0,0)ªOp`ÌàÉ é©»è
int is_origin_inner_triangle(int x1,int y1,int x2,int y2,int x3,int y3){
	double o1,o2,o3;
	o1=(double)x2*y3-(double)y2*x3;
	o2=(double)x3*y1-(double)y3*x1;
	o3=(double)x1*y2-(double)y1*x2;
	return o1*o2>0&&o2*o3>0;
}
int main(){
	int x1,y1,x2,y2,x3,y3,x4,y4,r;
	int inner1,inner2,inner3;
	int common1,common2,common3;
	for(;scanf("%d%d%d%d%d%d%d%d%d",&x1,&y1,&x2,&y2,&x3,&y3,&x4,&y4,&r),x1|y1;){
		x1-=x4,y1-=y4;
		x2-=x4,y2-=y4;
		x3-=x4,y3-=y4;
		inner1=x1*x1+y1*y1<=r*r;
		inner2=x2*x2+y2*y2<=r*r;
		inner3=x3*x3+y3*y3<=r*r;
		if(inner1&&inner2&&inner3){
			puts("b");
		}else if(!inner1&&!inner2&&!inner3){
			common1=has_common_circle_segment(x2,y2,x3,y3,r);
			common2=has_common_circle_segment(x3,y3,x1,y1,r);
			common3=has_common_circle_segment(x1,y1,x2,y2,r);
			if(common1||common2||common3){
				puts("c");
			}else{
				if(is_origin_inner_triangle(x1,y1,x2,y2,x3,y3))
					puts("a");
				else
					puts("d");
			}
		}else{
			puts("c");
		}
	}
	return 0;
}