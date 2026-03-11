#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <ctype.h>
#include <inttypes.h>

int compare_int(const void *a, const void *b){//qsort 整数比較<
	return *(int*)a - *(int*)b;
}
int compare_int2(const void *a, const void *b){//qsort 整数比較>
	return *(int*)b - *(int*)a;
}

int main(void){
	int h,w,k;
	char c[7][7]={'\0'};
	int i,j;
	int counts=0,count=0;
	int count2=0;
	int bit1,bit2;
	int flag[7][7]={0};
	int ans=0;

	scanf("%d %d %d\n",&h,&w,&k);
	scanf("\n");
	for(i=0;i<h;i++){
		for(j=0;j<w;j++){
			scanf("%c",&c[i][j]);
			if(c[i][j]=='#'){
				counts++;
			}
		}
		scanf("\n");
	}
	if(counts<k){
		printf("0");
		return 0;
	}else if(counts==k){
		printf("1");
		return 0;
	}
	for (bit1 = 0; bit1 < (1<<h); bit1++){
		count=0;
		for(i=0;i<h;i++){
			for(j=0;j<w;j++){
				flag[i][j]=0;
			}
		}
		if(bit1&1){
			for(i=0;i<w;i++){
				if(c[0][i]=='#'){
					count++;
					flag[0][i]=1;
				}
			}
		}
		if(h>1){
			if(bit1&2){
				for(i=0;i<w;i++){
					if(c[1][i]=='#'){
						count++;
						flag[1][i]=1;
					}
				}
			}
			if(h>2){
				if(bit1&4){
					for(i=0;i<w;i++){
						if(c[2][i]=='#'){
							count++;
							flag[2][i]=1;
						}
					}
				}
				if(h>3){
					if(bit1&8){
						for(i=0;i<w;i++){
							if(c[3][i]=='#'){
								count++;
								flag[3][i]=1;
							}
						}
					}
					if(h>4){
						if(bit1&16){
							for(i=0;i<w;i++){
								if(c[4][i]=='#'){
									count++;
									flag[4][i]=1;
								}
							}
						}if(h>5){
							if(bit1&32){
								for(i=0;i<w;i++){
									if(c[5][i]=='#'){
										count++;
										flag[5][i]=1;
									}
								}
							}
						}
					}
				}
			}
		}
		for (bit2 = 0; bit2 < (1<<w); bit2++){
			count2=0;
			if(bit2&1){
				for(i=0;i<h;i++){
					if(c[i][0]=='#'){
						if(flag[i][0]==0){
							count2++;
						}
					}
				}
			}
			if(w>1){
				if(bit2&2){
					for(i=0;i<h;i++){
						if(c[i][1]=='#'){
							if(flag[i][1]==0){
								count2++;
							}
						}
					}
				}
				if(w>2){
					if(bit2&4){
						for(i=0;i<h;i++){
							if(c[i][2]=='#'){
								if(flag[i][2]==0){
									count2++;
								}
							}
						}
					}
					if(w>3){
						if(bit2&8){
							for(i=0;i<h;i++){
								if(c[i][3]=='#'){
									if(flag[i][3]==0){
										count2++;
									}
								}
							}
						}
						if(w>4){
							if(bit2&16){
								for(i=0;i<h;i++){
									if(c[i][4]=='#'){
										if(flag[i][4]==0){
											count2++;
										}
									}
								}
							}
							if(w>5){
								if(bit2&32){
									for(i=0;i<h;i++){
										if(c[i][5]=='#'){
											if(flag[i][5]==0){
												count2++;
											}
										}
									}
								}
							}
						}
					}
				}
			}
			if(k==counts-count-count2){
				ans++;
			}
		}
	}

	printf("%d",ans);
	return 0;
}
