#include <stdio.h>

int main(void) {
	int N;
	while(scanf("%d",&N)==1 && N>0) {
		int i;
		char buf[16];
		int ibuf;
		int min=-2000000000,max=2000000000;
		int previsx=0;
		int prev=0;
		int noneflag=0;
		for(i=0;i<N;i++) {
			scanf("%s",buf);
			ibuf=0;
			if(buf[0]=='x') {
				if(previsx) {
					noneflag=1;
				} else {
					if(i>0) {
						if(i%2==0) {
							if(prev-1<max)max=prev-1;
						} else {
							if(prev+1>min)min=prev+1;
						}
					}
				}
				previsx=1;
			} else {
				sscanf(buf,"%d",&ibuf);
				if(i>0) {
					if(i%2==0) {
						if(previsx) {
							if(ibuf+1>min)min=ibuf+1;
						} else {
							if(prev<=ibuf)noneflag=1;
						}
					} else {
						if(previsx) {
							if(ibuf-1<max)max=ibuf-1;
						} else {
							if(prev>=ibuf)noneflag=1;
						}
					}
				}
				previsx=0;
			}
			prev=ibuf;
		}
		if(noneflag || min>max) {
			puts("none");
		} else if(min==max) {
			printf("%d\n",min);
		} else {
			puts("ambiguous");
		}
	}
	return 0;
}