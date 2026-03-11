#include <stdio.h>
 
int main(){
	int n, p[20], i, c=0;
 
	scanf("%d", &n);
 
	for(i=0; i<n; i++)
		scanf("%d", &p[i]);
 
	for(i=2; i<n; i++){
		if((p[i-2] < p[i-1] && p[i-1] < p[i]) || (p[i] < p[i-1] && p[i-1] < p[i-2]))
			c++;
	}
 
	printf("%d\n",c);
 
	return 0;
}