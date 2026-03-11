#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#define max(a, b)	(((a) > (b)) ? (a) : (b))		/* ２個の値の最大値 */
#define min(a, b)	(((a) < (b)) ? (a) : (b))		/* ２個の値の最小値 */
#define ENTER		printf("\n")					/* 改行プリント */
/********************************************************************************************************************************/
/* main *************************************************************************************************************************/
/********************************************************************************************************************************/
int DEBUG = 1;										/* デバッグプリント 提出時は0 */
int main()
{
	int		ans = 0;
	int		i,j,N,a[3],b[3],r[3],m;
	int		n[4] = {0};
	int		maxr[3];
	scanf("%d", &N);
	scanf("%d %d %d", &a[0], &a[1], &a[2]);
	scanf("%d %d %d", &b[0], &b[1], &b[2]);

	n[3] = N;
	
	r[0] = b[0] - a[0];
	r[1] = b[1] - a[1];
	r[2] = b[2] - a[2];

	if (r[0]>=r[1]) {
		if (r[1]>=r[2]) {
			maxr[0] = 0; maxr[1] = 1; maxr[2] = 2;
		} else {
			if (r[0]>=r[2])	{maxr[0] = 0; maxr[1] = 2; maxr[2] = 1; }
			else            {maxr[0] = 2; maxr[1] = 0; maxr[2] = 1; }
		}
	} else { // 1>0
		if (r[0]>=r[2]) {
			maxr[0] = 1; maxr[1] = 0; maxr[2] = 2;
		} else { // 1>0 , 2>0
			if (r[1]>=r[2]) {maxr[0] = 1; maxr[1] = 2; maxr[2] = 0; }
			else            {maxr[0] = 2; maxr[1] = 1; maxr[2] = 0; }
		}
	}
	for(i=0;i<3;i++) {
		m = maxr[i];
		if (r[m]<=0) continue;
		j = n[3] - (n[3]%a[m]);
		n[3] -= j;
		n[m] += j/a[m];
	}
		
	for(i=0;i<3;i++) {
		j = n[i]*b[i];
		n[3] += j;
		n[i] = 0;
	}

	r[0] = a[0] - b[0];
	r[1] = a[1] - b[1];
	r[2] = a[2] - b[2];

	if (r[0]>=r[1]) {
		if (r[1]>=r[2]) {
			maxr[0] = 0; maxr[1] = 1; maxr[2] = 2;
		} else {
			if (r[0]>=r[2])	{maxr[0] = 0; maxr[1] = 2; maxr[2] = 1; }
			else            {maxr[0] = 2; maxr[1] = 0; maxr[2] = 1; }
		}
	} else { // 1>0
		if (r[0]>=r[2]) {
			maxr[0] = 1; maxr[1] = 0; maxr[2] = 2;
		} else { // 1>0 , 2>0
			if (r[1]>=r[2]) {maxr[0] = 1; maxr[1] = 2; maxr[2] = 0; }
			else            {maxr[0] = 2; maxr[1] = 1; maxr[2] = 0; }
		}
	}

	for(i=0;i<3;i++) {
		m = maxr[i];
		if (r[m]<=0) continue;
		j = n[3] - (n[3]%b[m]);
		n[3] -= j;
		n[m] += j/b[m];
	}
		
	for(i=0;i<3;i++) {
		j = n[i]*a[i];
		n[3] += j;
		n[i] = 0;
	}

	printf("%d\n",n[3]);


	return 0;
}
