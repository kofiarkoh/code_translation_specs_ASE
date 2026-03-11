#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>
#define PI 3.141592653589793
#define MOD 1000000007

long int combination(long int n, long int r);

//qsort用、昇順
int compare_int(const void *a, const void *b)
{
	return *(int*)a-*(int*)b;
}
//qsort(data, 10, sizeof(int), compare_int)

long int combination(long int n, long int r)
{
	if(r>n-r) r = n-r;
	if(r == 0) return 1;
	else if(r == 1) return n;
	return (combination(n-1, r) + combination(n-1, r-1));
	//大きくなるとき
	/*
	long int tmp1, tmp2, tmp3;
	tmp1 = combination(n-1, r);
	if(tmp1>=MOD) tmp1 = tmp1%MOD;
	tmp2 = combination(n-1, r-1);
	if(tmp2>=MOD) tmp2 = tmp2%MOD;
	tmp3 = tmp1+tmp2;
	if(tmp3>=MOD) tmp3 = tmp3%MOD;
	return (tmp3);
	*/
}


int main(){
	char str[300001];
	scanf("%s", str);
	int q;
	scanf("%d", &q);

	int t, f;
	char c;
	for(int i=0; i<q; i++){
		int len = strlen(str);
		scanf("%d", &t);
		if(t == 1){
			char tmp[300001];
			strcpy(tmp, str);
			for(int i=0; i<len; i++){
				str[i] = tmp[len-1-i];
			}
		}
		else{
			scanf("%d %c", &f, &c);
			if(f==1){
				for(int i=len; i>0; i--){
					str[i] = str[i-1];
				}
				str[0] = c;
			}
			else{
				str[len] = c;
			}
		}
	}

	printf("%s\n", str);
	
	return 0;
}