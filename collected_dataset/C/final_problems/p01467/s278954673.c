#include <stdio.h>

int bit_count(int n) {
	n=((n & 0x2AAAAAAA)>>1 )+(n & 0x55555555);
	n=((n & 0x4CCCCCCC)>>2 )+(n & 0x33333333);
	n=((n & 0x70F0F0F0)>>4 )+(n & 0x0F0F0F0F);
	n=((n & 0x7F00FF00)>>8 )+(n & 0x00FF00FF);
	n=((n & 0x7FFF0000)>>16)+(n & 0x0000FFFF);
	return n;
}

int main(void) {
	int input_a;
	int input_b;
	int a[12]={0};
	int b[12]={0};
	int borrow[12]={0};
	int c[12]={0};
	int k;
	int tansaku;
	int out_c;
	int i;
	int max=0;
	scanf("%d%d%d",&input_a,&input_b,&k);
	for(i=0;input_a>0;i++,input_a/=10)a[i]=input_a%10;
	for(i=0;input_b>0;i++,input_b/=10)b[i]=input_b%10;
	for(tansaku=0;tansaku<(1<<10);tansaku++) {
		if(bit_count(tansaku)>k)continue;
		for(i=0;i<11;i++) {
			if(tansaku & (1<<i))borrow[i]=0;
			if(a[i]-borrow[i]>=b[i]) {
				c[i]=a[i]-borrow[i]-b[i];
				borrow[i+1]=0;
			} else {
				c[i]=a[i]-borrow[i]+10-b[i];
				borrow[i+1]=1;
			}
		}
		out_c=0;
		for(i=11;i>=0;i--)out_c=out_c*10+c[i];
		if(out_c>max)max=out_c;
	}
	printf("%d\n",max);
	return 0;
}