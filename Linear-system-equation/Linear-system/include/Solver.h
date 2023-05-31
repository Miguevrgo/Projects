
/* 
 * File:   Solver.h
 * Author: Miguel Angel de la vega rodriguez
 *
 * Created on 3 de mayo de 2023, 21:52
 */

#ifndef SOLVER_H
#define SOLVER_H

#include <string>
#include <cmath>
#include <iostream>

class Solver{
public:
    /**
     * @brief Construct a new Solver object with the s
     * @param size 
     */
    Solver(int size = 0);

    /**
     * @brief Construct a new Solver object
     * 
     * @param other 
     */
    Solver(const Solver& other);

    /**
     * @brief Destroy the Solver object
     * 
     */
    ~Solver();

    /**
     * @brief Operator() used to access members of matrix_coef as if it was
     * a matrix: matrix_coef(i,j)
     * @param row 
     * @param col 
     * @return double* 
     */
    double* operator()(int row, int col);

    /**
     * @brief Set the Matrix Coef object
     * 
     * @param matrix_coef 
     * @param size 
     */
    void setMatrixCoef(double matrix_coef[], int size);
    
    /**
     * @brief Set the Matrix Ind object
     * 
     * @param ind 
     * @param size 
     */
    void setMatrixInd(double ind[],int size);
    
    /**
     * @brief Solves given linear equation system with the use of iterative
     * Gauss-Seidel Method,
     * 
     * @param max_iterations 
     * @param tolerance 
     */
    void gaussSeidel(int max_iterations, double tolerance);
    
    /**
     * @brief Solves given linear equation system with the use of iterative
     * Jacobi Method,
     * 
     * @param max_iterations 
     * @param tolerance 
     */
    void jacobi(int max_iterations, double tolerance);

    /**
     * @brief Solves given linear equation system with the use of iterative
     * relaxation Method,
     * 
     * @param max_iterations 
     * @param tolerance 
     * @param w w used by the method
     */
    void relaxation(int max_iterations,double tolerance, double w);
    /**
     * @brief Solves the system by the given method
     * 
     * @param method gaussSeidel | Jacobi | Relaxation
     * @param max_iter 
     * @param tolerance 
     * @param w (optional)
     */
    void solveSystem(void (*method)(int,double), int max_iter, double tolerance);
    void solveSystem(void (*method)(int,double,double), int max_iter, double tolerance,double w);
    /**
     * @brief Calculates the inverse of the given matrix
     * 
     * @param matrix
     * @param size 
     */
    void inverseMatrix(double matrix[], int size);
    std::string toString() const;
private:
    unsigned int rows;
    unsigned int cols;
    double* matrix_coef; // Matrix coefficients
    double* matrix_sol; // Solution vector
    double* matrix_ind; // Independent vector

    void deallocate();

};


#endif /* SOLVER_H */

