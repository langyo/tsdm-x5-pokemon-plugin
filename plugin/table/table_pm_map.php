<?php

namespace pokemon;

use discuz_table;
use DB;

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

class table_pm_map extends discuz_table {

    public static function t() {
        static $_instance;
        if(!isset($_instance)) {
            $_instance = new self();
        }
        return $_instance;
    }

    public function __construct() {
        $this->_table = 'pm_map';
        $this->_pk = 'id';
        parent::__construct();
    }

    public function fetch_all() {
        return DB::fetch_all('SELECT * FROM %t ORDER BY id ASC', [$this->_table]);
    }

    public function fetch_by_region($region) {
        return DB::fetch_all('SELECT * FROM %t WHERE region=%s', [$this->_table, $region]);
    }
}
