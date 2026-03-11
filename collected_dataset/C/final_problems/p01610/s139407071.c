#include <stdio.h>

int main(void) {
	char banmen[6][10]={
		"..rr....",
		"..rr....",
		"ggyybbww",
		"ggyybbww",
		"..oo....",
		"..oo...."
	};
	char query[200];
	char t1,t2;
	int i;
	scanf("%s",query);
	for(i=0;query[i];i++) {
		switch(query[i]) {
			case 'U':
				t1=banmen[2][0];
				t2=banmen[2][1];
				banmen[2][0]=banmen[2][2];
				banmen[2][1]=banmen[2][3];
				banmen[2][2]=banmen[2][4];
				banmen[2][3]=banmen[2][5];
				banmen[2][4]=banmen[2][6];
				banmen[2][5]=banmen[2][7];
				banmen[2][6]=t1;
				banmen[2][7]=t2;

				t1=banmen[0][2];
				banmen[0][2]=banmen[1][2];
				banmen[1][2]=banmen[1][3];
				banmen[1][3]=banmen[0][3];
				banmen[0][3]=t1;
				break;
			case 'F':
				t1=banmen[1][2];
				t2=banmen[1][3];
				banmen[1][2]=banmen[3][1];
				banmen[1][3]=banmen[2][1];
				banmen[3][1]=banmen[4][3];
				banmen[2][1]=banmen[4][2];
				banmen[4][3]=banmen[2][4];
				banmen[4][2]=banmen[3][4];
				banmen[2][4]=t1;
				banmen[3][4]=t2;

				t1=banmen[2][2];
				banmen[2][2]=banmen[3][2];
				banmen[3][2]=banmen[3][3];
				banmen[3][3]=banmen[2][3];
				banmen[2][3]=t1;
				break;
			case 'R':
				t1=banmen[0][3];
				t2=banmen[1][3];
				banmen[0][3]=banmen[2][3];
				banmen[1][3]=banmen[3][3];
				banmen[2][3]=banmen[4][3];
				banmen[3][3]=banmen[5][3];
				banmen[4][3]=banmen[3][6];
				banmen[5][3]=banmen[2][6];
				banmen[3][6]=t1;
				banmen[2][6]=t2;

				t1=banmen[2][4];
				banmen[2][4]=banmen[3][4];
				banmen[3][4]=banmen[3][5];
				banmen[3][5]=banmen[2][5];
				banmen[2][5]=t1;
				break;
			case 'D':
				t1=banmen[3][6];
				t2=banmen[3][7];
				banmen[3][6]=banmen[3][4];
				banmen[3][7]=banmen[3][5];
				banmen[3][4]=banmen[3][2];
				banmen[3][5]=banmen[3][3];
				banmen[3][2]=banmen[3][0];
				banmen[3][3]=banmen[3][1];
				banmen[3][0]=t1;
				banmen[3][1]=t2;

				t1=banmen[4][2];
				banmen[4][2]=banmen[5][2];
				banmen[5][2]=banmen[5][3];
				banmen[5][3]=banmen[4][3];
				banmen[4][3]=t1;
				break;
			case 'B':
				t1=banmen[0][2];
				t2=banmen[0][3];
				banmen[0][2]=banmen[2][5];
				banmen[0][3]=banmen[3][5];
				banmen[2][5]=banmen[5][3];
				banmen[3][5]=banmen[5][2];
				banmen[5][3]=banmen[3][0];
				banmen[5][2]=banmen[2][0];
				banmen[3][0]=t1;
				banmen[2][0]=t2;

				t1=banmen[2][6];
				banmen[2][6]=banmen[3][6];
				banmen[3][6]=banmen[3][7];
				banmen[3][7]=banmen[2][7];
				banmen[2][7]=t1;
				break;
			case 'L':
				t1=banmen[4][2];
				t2=banmen[5][2];
				banmen[4][2]=banmen[2][2];
				banmen[5][2]=banmen[3][2];
				banmen[2][2]=banmen[0][2];
				banmen[3][2]=banmen[1][2];
				banmen[0][2]=banmen[3][7];
				banmen[1][2]=banmen[2][7];
				banmen[3][7]=t1;
				banmen[2][7]=t2;

				t1=banmen[2][0];
				banmen[2][0]=banmen[3][0];
				banmen[3][0]=banmen[3][1];
				banmen[3][1]=banmen[2][1];
				banmen[2][1]=t1;
				break;
		}
	}
	for(i=0;i<6;i++)puts(banmen[i]);
	return 0;
}