/*
 * main.c
 *
 *  Created on: 2020/03/28
 *      Author: family
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define MAX(a,b) (a > b ? a : b)
#define MIN(a,b) (a > b ? b : a)
typedef long long int ll;
typedef unsigned long long int ull;

int sort_inc(const void *a, const void *b) { return (*(int*)a - *(int*)b);}
int sort_dec(const void* a, const void* b) { return (*(int*)b - *(int*)a);}
int sort_dec_ll(const void *a, const void *b) {
    ll da = *(ll*)a, db = *(ll*)b;    int val = 0;
    if(da > db) { val = -1; }
    else if (da == db) { val = 0; }
    else { val = 1; }
    return val;
}
int sort_inc_ll(const void *a, const void *b) {
    ll da = *(ll*)a, db = *(ll*)b;    int val = 0;
    if(da > db) { val = 1; }
    else if (da == db) { val = 0; }
    else { val = -1; }
    return val;
}
int sort_dic(const void *a, const void *b) {
    char *pa = (char *)a;    char *pb = (char *)b;    int i = 0, val = 0, N = 10;
    for (i = 0; i < N; i++) {
    	char da = pa[i], db = pb[i];
    	if (da == db) continue;
  		if (da > db) val = 1; else val = -1;
        break;
    }
    return val;
}

void yesno(int ans) {
	if (ans == 1) {	printf("Yes\n");
	} else {		printf("No\n");	}
	return;
}

void okng(int ans) {
	if (ans == 1) { printf("OK\n");
	} else { 		printf("NG\n");	}
	return;
}

int AB[100001][2] = {0};
int city[100002] = {0};
int main()
{
	int N = 0, M = 0;
	scanf("%d %d", &N, &M);

	int i = 0;
	memset(AB, M, sizeof(int));
	for (i = 0; i < M; i++) {
		int a = 0, b = 0;
		scanf("%d %d", &a, &b);
		AB[i][0] = a;
		AB[i][1] = b;
		city[a]++;
		city[b]++;
	}

	int min_road = N - 1;
	int ans = min_road - M;
	if (ans < 0) {
		ans = 0;
	}
	int ans2 = 0;
	for (i = 1; i < N; i++) {
		if (city[i] == 0) {
			ans2++;
		}
	}
	ans = MAX(ans, ans2);

	printf("%d\n", ans);
	return 0;
}
