/*
 * main.c
 *
 *  Created on: 2019/05/25
 *      Author: family
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int main()
{
	int r = 0, D = 0, x = 0, ans = 0;
	int i = 0;
	scanf("%d %d %d", &r, &D, &x);
	for(i = 0; i < 10; i++) {
		ans = (r * x) - D;
		printf("%d\n",ans);
		x = ans;
	}
    return 0;
}
