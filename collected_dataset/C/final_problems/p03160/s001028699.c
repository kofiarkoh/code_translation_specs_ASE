#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int h_height[100000+1] = {0};
typedef struct {
	int len;
	int point;
} point_len_t;

unsigned int get_len(int point, int next, int max)
{
	if ((point + next) >= max) {
		next = max - point;
	}

	unsigned long long int value = (unsigned long long int)abs(h_height[point + next] - h_height[point]);
//	printf("%d %d\n", h_height[point + next], h_height[point]);
	return value;
}


point_len_t get_small(int point, int max)
{
	point_len_t ret_val = {0};
	unsigned int value = get_len(point, 1, max);
	unsigned int value2 = get_len(point, 2, max);
	if (value >= value2) {
		ret_val.len = value2;
		ret_val.point = point+2;
	} else {
		ret_val.len = value;
		ret_val.point = point+1;
	}
	return ret_val;
}

int main()
{
    unsigned int n_foot = 0;
    unsigned int loop = 0;
    unsigned long long int sum = 0;
    unsigned long long int sum2 = 0;
    point_len_t tmp_val = {0};

    scanf("%d", &n_foot);

    while(loop < n_foot ) {
    	scanf("%d", &h_height[loop]);
    	loop++;
    }

	sum += get_len(0, 1, n_foot-1);
	tmp_val.point = 1;
    while(1) {
        tmp_val = get_small(tmp_val.point,n_foot-1);
        sum += tmp_val.len;
//        printf("point=%d\n",tmp_val.point);
        if (tmp_val.point >= n_foot-1) {
        	break;
        }
    }

	sum2 += get_len(0, 2, n_foot-1);
	tmp_val.point = 2;
    while(1) {
        tmp_val = get_small(tmp_val.point,n_foot-1);
        sum2 += tmp_val.len;
//        printf("point=%d\n",tmp_val.point);
        if (tmp_val.point >= n_foot-1) {
        	break;
        }
    }

    if(sum>sum2) {
    	sum = sum2;
    }
    printf("%llu\n",sum);

    return 0;
}
