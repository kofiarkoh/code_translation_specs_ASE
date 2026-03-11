#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

/*
 * [Basic Idea] Use DP
 */

typedef unsigned int pos_value_t;
typedef unsigned long tree_length_t;

typedef struct dp_table_entry {
    tree_length_t dpe_min_tree_length;
    pos_value_t dpe_x;
    pos_value_t dpe_y;
} dp_table_entry;

#define NUM_POINTS_MAX 1000

int
load_data(FILE *fp, int num_points_max, pos_value_t x[], pos_value_t y[])
{
    int n;
    unsigned int i;

    if (1 != fscanf(fp, "%u\n", &n)) return -1;
    if (num_points_max < n) return -1;

    for (i = 0; i < n; i++) {
        pos_value_t xi, yi;

        if (2 != fscanf(fp, "%u %u\n", &xi, &yi)) return -1;
        x[i] = xi, y[i] = yi;
    }
    return n;
}

void
init_dp_table(int n, dp_table_entry **table, pos_value_t x[], pos_value_t y[])
{
    int i, j;
    const tree_length_t LENGTH_MAX = ULONG_MAX;

    for (i = 0; i < n; i++) {
        table[i][i].dpe_min_tree_length = 0;
        table[i][i].dpe_x = x[i];
        table[i][i].dpe_y = y[i];
    }
    for (i = 0; i < n; i++) {
        for (j = i+1; j < n; j++) {
            table[i][j].dpe_min_tree_length = LENGTH_MAX;
        }
    }
}

tree_length_t
length_of_tree_between(dp_table_entry **table, int i, int d, int j)
{
    dp_table_entry *ip, *jp;
    tree_length_t length = 0;

    ip = &(table[i][d]);
    jp = &(table[d+1][j]);

    length += ip->dpe_min_tree_length;
    length += jp->dpe_min_tree_length;
    length += jp->dpe_x - ip->dpe_x;
    length += ip->dpe_y - jp->dpe_y;

    return length;
}

void
fill_dp_table(int n, dp_table_entry **table)
{
    int i, j, k;

    /*
     * table[0][1], table[1][2], ..., table[n-2][n-1]
     * table[0][2], table[1][3], ..., table[n-3][n-1]
     * :
     * table[0][n-3], table[1][n-2], table[2][n-1]
     * table[0][n-2], table[1][n-1]
     * table[0][n-1]
     */
    for (k = 1; k < n; k++) {
        for (i = 0, j = k; j < n; i++, j++) {
            /* table[0][k], table[1][1+k], ..., table[n-1-k][n-1] */
            int d; /* division point */
            dp_table_entry *ep = &(table[i][j]);

            for (d = i; d < j; d++) {
                tree_length_t length = length_of_tree_between(table, i, d, j);
                if (ep->dpe_min_tree_length <= length) continue;

                ep->dpe_min_tree_length = length;
                ep->dpe_x = table[i][d].dpe_x;
                ep->dpe_y = table[d+1][j].dpe_y;
            }
        }
    }
}

dp_table_entry**
dp_table_alloc(int n)
{
    int i;
    dp_table_entry **table;
    
    if (!(table = malloc(n * sizeof(dp_table_entry*)))) {
        return NULL;
    }
    memset(table, 0, (n * sizeof(dp_table_entry*)));
    for (i = 0; i < n; i++) {
        if (!(table[i] = malloc(n * sizeof(dp_table_entry)))) {
            goto free_exit;
        }
    }
    return table;

free_exit:
    for (i = 0; i < n; i++) {
        if (!table[i]) continue;
        free(table[i]);
        table[i] = NULL;
    }
    free(table);
    table = NULL;
    return NULL;
}

void
dp_table_free(dp_table_entry** table, int n)
{
    int i;

    for (i = 0; i < n; i++) {
        free(table[i]);
        table[i] = NULL;
    }
    free(table);
}

tree_length_t
calc_total_length(int n, pos_value_t x[], pos_value_t y[])
{
    tree_length_t length;
    dp_table_entry **table;
    
    if (!(table = dp_table_alloc(n))) {
        return 0; /* pending: should return negative value? */
    }
    init_dp_table(n, table, x, y);
    fill_dp_table(n, table);
    length = table[0][n-1].dpe_min_tree_length;
    
    dp_table_free(table, n);
    table = NULL;

    return length;
}

int
main(int argc, char *argv[])
{
    pos_value_t x[NUM_POINTS_MAX], y[NUM_POINTS_MAX];
    int n;

    if ((n = load_data(stdin, NUM_POINTS_MAX, x, y)) <= 0) {
        return 1;
    }
    printf("%lu\n", calc_total_length(n, x, y));

    return 0;
}