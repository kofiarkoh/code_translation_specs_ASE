#define _CRT_SECURE_NO_WARNINGS
#define TRUE 1;
#define FALSE 0;

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

int GetBit(int mask, int v){
	return (mask >> v) & 1;
}

int AddBit(int mask, int v){
	return mask | (1 << v);
}

int Task(int* pointRib, int n){
	int* dP=NULL;
	int maxMask = pow((double)2, (double)n);
	dP = (int*)calloc((maxMask) * n, sizeof(int));
	int mask = 0;
	int vertex = 0;
	short IO = TRUE;
	int i;
	for (mask = 1; mask < maxMask; mask++){
		for (vertex = 0; vertex < n; vertex++){
			for (i = 0; i < n; i++){
				if (1 != GetBit(mask, i) && 1 == GetBit(mask, vertex) && 0 != pointRib[vertex*n + i] && (1 == mask || (1 < mask && 0 != dP[mask*n + vertex]))){
					dP[AddBit(mask, i)*n + i] = pointRib[vertex*n + i] + dP[mask*n + vertex];
				}
			}
		}
	}
	int min = 0;
	for (vertex = 0; vertex < n; vertex++){
		if (0 != dP[(maxMask - 1)*n + vertex] && 0 != pointRib[vertex*n]){
			if (0 == min || (min>(dP[(maxMask - 1)*n + vertex] + pointRib[vertex*n]))){
				min = dP[(maxMask - 1)*n + vertex] + pointRib[vertex*n];
			}
		}
	}
	return (0 != min) ? min : -1;
}

int main(){
	int n, m;
	scanf("%d %d", &n, &m);
	int i;
	int* rib = NULL;
	rib = calloc(n * n, sizeof(int));
	int x, y, weight;
	for (i = 0; i < m; i++){
		scanf("%d %d %d", &x, &y, &weight);
		rib[x*n + y] = weight;
	}
	printf("%d\n", Task(rib, n));

	return 0;
}